#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let db = ::hokun::setup_db().await?;
    let config = ::toasty_cli::Config::new();
    let toasty_cli = ::toasty_cli::ToastyCli::with_config(db, config);
    toasty_cli.parse_and_run().await?;
    Ok(())
}
