// Project structure inspired by: https://github.com/TatriX/realworld-rust-rocket

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _ = ciservice::rocket().launch().await?;
    Ok(())
}
