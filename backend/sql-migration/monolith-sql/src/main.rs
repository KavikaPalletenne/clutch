use anyhow::Result;

mod auth;
mod errors;
mod handler;
mod models;
mod service;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Hello, world!");

    Ok(())
}
