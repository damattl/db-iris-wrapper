use chrono::NaiveDate;
use rocket::{form::{self, FromFormField}, request::FromParam};

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

impl<'r> FromFormField<'r> for DateParam {
    fn from_value(field: rocket::form::ValueField<'r>) -> rocket::form::Result<'r, Self> {
        NaiveDate::parse_from_str(field.value, DATE_FMT)
            .map(DateParam)
            .map_err(|_| form::Error::validation("invalid date").into())
    }
}
