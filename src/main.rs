pub mod error;
pub mod commands;

#[rocket::main]
async fn main() -> crate::error::BlogrsResult {
    let args: Vec<String> = std::env::args().collect();
    let commands = commands::parse_args(&args[1..].to_vec())?;

    println!("{:?}", commands);

    Ok(())
}
