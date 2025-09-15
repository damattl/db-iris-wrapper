#[derive(Debug, Clone)]
pub enum StatusCodeType {
    TravelInfo,
    Quality,
    Unknown,
}

impl StatusCodeType {
    pub fn as_str(&self) -> &'static str {
        match self {
            StatusCodeType::TravelInfo => "R",
            StatusCodeType::Quality => "Q",
            StatusCodeType::Unknown => "U",
        }
    }
    pub fn as_string(&self) -> String {
        match self {
            StatusCodeType::TravelInfo => "R".to_string(),
            StatusCodeType::Quality => "Q".to_string(),
            StatusCodeType::Unknown => "U".to_string(),
        }
    }
}

impl From<&str> for StatusCodeType {
    fn from(value: &str) -> StatusCodeType {
        match value {
            "R" => StatusCodeType::TravelInfo,
            "Q" => StatusCodeType::Quality,
            other => {
                warn!("Unknown StatusCodeType: {}", other);
                StatusCodeType::Unknown
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct StatusCode {
    pub code: i16,
    pub c_type: Option<StatusCodeType>,
    pub long_text: String,
}
