pub mod app;
pub mod blog;
pub mod commands;
pub mod error;
pub mod index;

#[rocket::main]
async fn main() -> crate::error::BlogrsResult {
    let args: Vec<String> = std::env::args().collect();
    let commands = commands::parse_args(&args[1..].to_vec())?;
    let config = app::AppConfig::from_commands(commands);
    app::App::with_config(config).run().await?;

    Ok(())
}
