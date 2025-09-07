use chrono::NaiveDate;
use rocket::request::FromParam;

const DATE_FMT: &str = "%y%m%d"; // e.g., 2025-09-07

pub struct DateParam(pub NaiveDate);

impl<'r> FromParam<'r> for DateParam {
    type Error = &'r str;

    fn from_param(param: &'r str) -> Result<Self, Self::Error> {
        NaiveDate::parse_from_str(param, DATE_FMT)
            .map(DateParam)
            .map_err(|_| param)
    }
}
