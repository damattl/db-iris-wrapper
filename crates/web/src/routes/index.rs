use std::{env, path::{Path, PathBuf}};


use rocket::{fs::{NamedFile}, get, routes, Route};


#[get("/<path..>", rank = 20)]
async fn spa_index(path: PathBuf) -> Option<NamedFile> {
    // Only fall back if no API or static file matched

    if path.starts_with("v1") {
        return None;
    }

    let index_html_path = format!("{}/index.html", get_static_path());
    NamedFile::open(Path::new(&index_html_path)).await.ok()
}

pub fn get_static_path() -> String {
    env::var("STATIC_FILES_PATH").unwrap_or("static".to_string())
}

pub fn routes() -> Vec<Route> {
    routes![
        spa_index,
    ]
}
