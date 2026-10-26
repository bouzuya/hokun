#[derive(::clap::Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(::clap::Subcommand)]
enum Command {
    CreateUser(CreateUserArgs),
    Serve,
}

#[derive(::clap::Args)]
struct CreateUserArgs {
    #[arg(long)]
    name: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let cli = <Cli as ::clap::Parser>::parse();

    let mut db = ::hokun::setup_db().await?;
    match cli.command.unwrap_or(Command::Serve) {
        Command::Serve => {
            let app = ::hokun::app::App::new(db).await?;
            let router_service = ::hokun::app::router(app)?;
            ::topcoat::start(router_service).await?;
        }
        Command::CreateUser(CreateUserArgs { name }) => {
            ::hokun::data::User::create()
                .id(::uuid::Uuid::new_v4().to_string())
                .name(name)
                .exec(&mut db)
                .await?;
        }
    }
    Ok(())
}
