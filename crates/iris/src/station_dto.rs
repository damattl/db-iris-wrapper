/*
 * Example XML
 * <stations>
 *  <station p="13b|11a|11b|5a/b|7a/b|7a|7b|5a|5b|S|13a/b|11a/b|14a|14b|11|12a|12|12b|13|14|8a/b|6a/b|8a|8b|6a|6b|5|6|7|8|12a/b|14a/b|13a"
 *      meta="694887|8071065|8076116|8098549"
 *      name="Hamburg Hbf"
 *      eva="8002549"
 *      ds100="AH" db="true" creationts="25-09-02 10:42:08.821"/>
 * </stations>
 */

use chrono::NaiveDateTime;
use serde::{Deserialize, Deserializer, Serialize};



#[derive(thiserror::Error, Debug)]
pub enum IRISStationError {
    #[error(transparent)]
    Network(#[from] Box<ureq::Error>),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Xml(#[from] quick_xml::DeError),
    #[error("station not found: {0}")]
    NotFound(String),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error("invalid src format {0}")]
    InvalidSourceFormat(String),
}


#[derive(Debug, Deserialize, Serialize)]
pub struct Stations {
    #[serde(rename = "station")]
    pub stations: Vec<IRISStation>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct IRISStation {
    #[serde(rename = "@p", default, deserialize_with = "de_pipe_list")]
    pub platforms: Option<Vec<String>>,

    #[serde(rename = "@meta", default, deserialize_with = "de_pipe_list")]
    pub meta: Option<Vec<String>>,

    #[serde(rename = "@name")]
    pub name: String,

    #[serde(rename = "@eva")]
    pub eva: String,

    #[serde(rename = "@ds100")]
    pub ds100: String,

    #[serde(rename = "@db")]
    pub db: bool,

    #[serde(rename = "@creationts", deserialize_with = "de_ts")]
    pub creation_ts: NaiveDateTime,
}

fn de_pipe_list<'de, D>(de: D) -> Result<Option<Vec<String>>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::<String>::deserialize(de)?;

    Ok(opt.map(|s| s.split('|').map(|x| x.to_string()).collect()))
}

fn de_ts<'de, D>(de: D) -> Result<NaiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(de)?;
    // "25-09-02 10:42:08.821" → %y-%m-%d %H:%M:%S%.3f
    NaiveDateTime::parse_from_str(&s, "%y-%m-%d %H:%M:%S%.3f")
        .or_else(|_| NaiveDateTime::parse_from_str(&s, "%y-%m-%d %H:%M:%S"))
        .map_err(serde::de::Error::custom)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StationInfosPayload {
    pub stations: Vec<StationInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StationInfo {
    pub eva: u32,
    pub ds100: Option<String>,
    pub lat: f64,
    pub lon: f64,
    pub name: String,
    pub is_active_ris: bool,
    pub is_active_iris: bool,
    pub meta_evas: Vec<u64>,
    pub available_transports: Vec<String>,
    pub number_of_events: Option<u64>,
}
