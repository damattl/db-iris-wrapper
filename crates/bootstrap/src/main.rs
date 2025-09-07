use web::build;

#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    build().launch().await?;
    Ok(())
}
