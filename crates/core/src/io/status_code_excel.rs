use calamine::{open_workbook, DeError, HeaderRow, RangeDeserializerBuilder, Reader, Xlsx, XlsxError};
use serde::Deserialize;

use crate::model::{StatusCode, StatusCodeType};

#[derive(Deserialize)]
#[derive(Debug, Clone)]
pub struct StatusCodeRow {
    #[serde(rename = "Code")]
    pub code: i16,
    #[serde(rename = "Typ")]
    pub c_type: Option<String>,
    #[serde(rename = "Langtext (neu)")]
    pub long_text: String,
}


impl StatusCodeRow {
    pub fn to_model(&self) -> StatusCode {
        StatusCode {
            code: self.code,
            c_type: self.c_type.as_deref().map(StatusCodeType::from),
            long_text: self.long_text.clone(),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ExcelImportError {
    #[error(transparent)]
    IO(#[from] XlsxError),
    #[error("Sheet not found")]
    SheetNotFound,
    #[error(transparent)]
    ParsingError(#[from] DeError),
}



pub fn get_codes_from_excel(path: &str) -> Result<Vec<StatusCodeRow>, ExcelImportError> {
    let mut workbook: Xlsx<_> = open_workbook(path)?;

    let Some(Ok(range)) = workbook.with_header_row(HeaderRow::Row(1)).worksheet_range_at(0) else {
        error!("Sheet not found!");
        return Err(ExcelImportError::SheetNotFound);
    };

    let iter = RangeDeserializerBuilder::with_deserialize_headers::<StatusCodeRow>().from_range(&range)?;

    let mut codes = Vec::new();

    // Excel starts at row 3, so skip the first two rows
    for result in iter {
        let row: StatusCodeRow =  match result {
            Ok(row) => row,
            Err(DeError::Custom(_)) => continue,
            Err(err) => return Err(ExcelImportError::ParsingError(err)),
        };
        codes.push(row);
    }

    Ok(codes)
}
