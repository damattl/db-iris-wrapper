use chrono::NaiveDateTime;
use serde::{Deserialize, Deserializer, Serialize};

#[derive(thiserror::Error, Debug)]
pub enum GetTimetableError {
    #[error(transparent)]
    Network(#[from] Box<ureq::Error>),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Xml(#[from] quick_xml::DeError),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Timetable {
    #[serde(rename = "@station")]
    pub station: String,
    #[serde(rename = "@eva")]
    pub eva: Option<String>, // new: timetable-level EVA (optional)

    #[serde(rename = "s")]
    pub stops: Vec<Stop>,
}

// ---------- Stop (s) ----------

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Stop {
    #[serde(rename = "@id")]
    pub id: String,

    #[serde(rename = "@eva")]
    pub eva: Option<String>, // present in your new file

    // Train meta (sometimes present, sometimes absent)
    #[serde(default)]
    pub tl: Option<TrainLine>,

    // Messages directly under <s> (h/f/d/q; disruptions, info, etc.)
    #[serde(rename = "m", default)]
    pub msgs: Vec<Msg>,

    // Either/both may exist; Movement holds nested <m> too
    #[serde(rename = "ar")]
    #[serde(default)]
    pub arrival: Option<Movement>,

    #[serde(rename = "dp")]
    #[serde(default)]
    pub departure: Option<Movement>,
}

// ---------- TrainLine (tl) ----------

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TrainLine {
    #[serde(rename = "@f")]
    pub f: Option<String>, // e.g., "F","N","D"
    #[serde(rename = "@t")]
    pub t: Option<String>, // often "p"
    #[serde(rename = "@o")]
    pub operator: Option<String>, // "80","R1","RISSDZ",...
    #[serde(rename = "@c")]
    pub category: Option<String>, // "ICE","RE","RB","EC","DZ","ME","NBE",...
    #[serde(rename = "@n")]
    pub number: Option<String>, // train number
}

// ---------- Movement (ar/dp) ----------

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Movement {
    // planned or current time (packed "yymmddHHMM")
    #[serde(rename = "@pt", default, deserialize_with = "opt_pt")]
    pub planned: Option<NaiveDateTime>,
    #[serde(rename = "@ct", default, deserialize_with = "opt_pt")]
    pub current: Option<NaiveDateTime>,

    // platform/line/flags
    #[serde(rename = "@pp")]
    pub platform: Option<String>,
    #[serde(rename = "@l")]
    pub line: Option<String>,
    #[serde(rename = "@hi")]
    pub hi: Option<u8>,

    // full / changed path (pipe-separated)
    #[serde(rename = "@ppth", default, deserialize_with = "opt_pipe_list")]
    pub ppth: Option<Vec<String>>,
    #[serde(rename = "@cpth", default, deserialize_with = "opt_pipe_list")]
    pub cpth: Option<Vec<String>>,

    // additional attributes seen in file
    #[serde(rename = "@cs")]
    pub cs: Option<String>,
    #[serde(rename = "@clt", default, deserialize_with = "opt_pt")]
    pub clt: Option<NaiveDateTime>,
    #[serde(rename = "@wings")]
    pub wings: Option<String>,

    // nested messages within ar/dp
    #[serde(rename = "m", default)]
    pub msgs: Vec<Msg>,
}

// ---------- Message (m) ----------

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Msg {
    #[serde(rename = "@id")]
    pub id: Option<String>, // not always present

    // type: h (Hinweis), d (delay/changed op), f (finalized/neutral), q (quality), etc.
    #[serde(rename = "@t")]
    pub kind: Option<String>,

    // time bounds for 'h' messages (packed)
    #[serde(rename = "@from", default, deserialize_with = "opt_pt")]
    pub from: Option<NaiveDateTime>,
    #[serde(rename = "@to", default, deserialize_with = "opt_pt")]
    pub to: Option<NaiveDateTime>,

    // category/priority/codes
    #[serde(rename = "@cat")]
    pub cat: Option<String>, // e.g., "Störung", "Information"
    #[serde(rename = "@pr")]
    pub pr: Option<u8>, // priority
    #[serde(rename = "@c")]
    pub code: Option<i32>, // numeric code (e.g., 31, 36, 95, 1000)

    // timestamps: packed + human-readable
    #[serde(rename = "@ts", default, deserialize_with = "opt_pt")]
    pub ts: Option<NaiveDateTime>,
    #[serde(rename = "@ts-tts", default, deserialize_with = "opt_tts")]
    pub ts_tts: Option<NaiveDateTime>,
}

// ---------- Deserializers ----------

fn de_pipe_list<'de, D>(de: D) -> std::result::Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(de)?;
    Ok(s.split('|')
        .filter(|v| !v.is_empty())
        .map(str::to_string)
        .collect())
}

fn opt_pipe_list<'de, D>(de: D) -> std::result::Result<Option<Vec<String>>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(Some(de_pipe_list(de)?))
}

fn opt_pt<'de, D>(de: D) -> std::result::Result<Option<NaiveDateTime>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(Some(de_pt_yymmdd_hhmm(de)?))
}

fn de_tts<'de, D>(de: D) -> std::result::Result<NaiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(de)?;
    // "25-09-05 12:08:03.550"
    NaiveDateTime::parse_from_str(&s, "%y-%m-%d %H:%M:%S%.f")
        .or_else(|_| NaiveDateTime::parse_from_str(&s, "%y-%m-%d %H:%M:%S"))
        .map_err(serde::de::Error::custom)
}

fn opt_tts<'de, D>(de: D) -> std::result::Result<Option<NaiveDateTime>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(Some(de_tts(de)?))
}

fn de_pt_yymmdd_hhmm<'de, D>(de: D) -> std::result::Result<NaiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(de)?;
    // Example: "2509051025" = 2025-09-05 10:25
    NaiveDateTime::parse_from_str(&s, "%y%m%d%H%M").map_err(serde::de::Error::custom)
}
