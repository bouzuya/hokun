#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let db = ::hokun::setup_db().await?;
    let app = ::hokun::app::App::new(db).await?;
    let router_service = ::hokun::app::router(app)?;
    ::topcoat::start(router_service).await?;
    Ok(())
}
