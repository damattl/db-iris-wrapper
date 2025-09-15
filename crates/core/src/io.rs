// Contains all functions to get data thats neither from a database nor the IRIS API

use std::env;

use crate::{io::status_code_excel::{get_codes_from_excel, ExcelImportError}, model::status_code::StatusCode};
mod status_code_excel;


#[derive(thiserror::Error, Debug)]
pub enum IOError {
    #[error("invalid src format {0}")]
    InvalidSourceFormat(String),
    #[error(transparent)]
    ExcelError(#[from] ExcelImportError),
}

pub fn get_status_codes() -> Result<Vec<StatusCode>, IOError> {
    let status_codes_src = env::var("STATUS_CODES_SRC")
        .unwrap_or("EXCEL:./codes.xlsx".to_string());

    let parts: Vec<String> = status_codes_src.splitn(2, ':').map(|p| p.to_string()).collect();
    let src_type = parts.first().ok_or(IOError::InvalidSourceFormat(status_codes_src.clone()))?;
    let src = parts.get(1).ok_or(IOError::InvalidSourceFormat(status_codes_src.clone()))?;

    println!("{}", status_codes_src);

    match src_type.as_str() {
        "EXCEL" => {
            Ok(get_codes_from_excel(src)?.iter().map(|c| c.to_model()).collect())
        },
        _ => {
            Err(IOError::InvalidSourceFormat(status_codes_src.clone()))
        }
    }
}
