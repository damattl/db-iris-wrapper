// Contains all functions to get data thats neither from a database nor the IRIS API

use std::env;

use crate::{io::status_code_excel::{get_codes_from_excel, ExcelImportError}, model::StatusCode};
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


#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::path::PathBuf;
    use std::sync::{Mutex, OnceLock};

    static ENV_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

    fn env_lock() -> &'static Mutex<()> {
        ENV_LOCK.get_or_init(|| Mutex::new(()))
    }

    fn env_guard() -> std::sync::MutexGuard<'static, ()> {
        env_lock().lock().unwrap_or_else(|poison| poison.into_inner())
    }

    #[test]
    fn get_status_codes_rejects_unknown_source() {
        let _guard = env_guard();
        env::set_var("STATUS_CODES_SRC", "JSON:./unknown.json");

        let err = get_status_codes().unwrap_err();
        match err {
            IOError::InvalidSourceFormat(src) => assert!(src.starts_with("JSON:")),
            other => panic!("unexpected error: {:?}", other),
        }

        env::remove_var("STATUS_CODES_SRC");
    }

    #[test]
    fn get_status_codes_uses_default_excel_path() {
        let _guard = env_guard();
        let original_dir = env::current_dir().expect("current_dir");
        let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let workspace_root = manifest_dir
            .parent()
            .and_then(|p| p.parent())
            .expect("workspace root")
            .to_path_buf();
        env::set_current_dir(&workspace_root).expect("set_current_dir workspace root");

        env::remove_var("STATUS_CODES_SRC");

        let result = get_status_codes().expect("default Excel import should succeed");
        assert!(!result.is_empty());

        env::set_current_dir(original_dir).expect("restore current_dir");
    }
}
