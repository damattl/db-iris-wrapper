use std::env;

use wrapper_core::io::get_status_codes;

#[test]
fn test_get_codes_from_excel() {
    // Setup
    let _ = pretty_env_logger::try_init();
    env::set_var("STATUS_CODES_SRC", "EXCEL:./tests/data/codes.xlsx");

    // Test
    let result = get_status_codes().unwrap();
    assert!(result.last().is_some());
    assert!(result.last().unwrap().code == 99);
}
