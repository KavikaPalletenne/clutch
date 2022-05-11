use anyhow::Result;

mod models;
mod errors;
mod service;
mod handler;
mod auth;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Hello, world!");



    Ok(())
}
