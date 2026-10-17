mod app;
mod components;
mod data;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    ::topcoat::start(self::app::router().await?).await?;
    Ok(())
}
