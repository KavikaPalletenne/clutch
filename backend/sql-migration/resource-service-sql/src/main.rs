use anyhow::Result;

mod models;
mod errors;
mod service;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Hello, world!");



    Ok(())
}
