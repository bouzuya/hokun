pub mod app;
pub mod components;
pub mod data;

pub async fn setup_db() -> Result<::toasty::Db, Box<dyn std::error::Error + Send + Sync>> {
    let is_topcoat_dev = std::env::var_os("TOPCOAT_DEV_URL").is_some();

    let db = ::toasty::Db::builder()
        .models(::toasty::models!(
            crate::data::Book,
            crate::data::Page,
            crate::data::Report,
            crate::data::Session,
            crate::data::User
        ))
        .connect(if is_topcoat_dev {
            "sqlite::memory:"
        } else {
            "sqlite:./hokun.db"
        })
        .await?;

    if is_topcoat_dev {
        db.push_schema().await?;
        setup_dummy_data(db.clone()).await?;
    }

    Ok(db)
}

async fn setup_dummy_data(mut db: ::toasty::Db) -> Result<(), ::toasty::Error> {
    use crate::data::Books;
    use crate::data::Pages;
    use crate::data::User;
    use crate::data::Users;

    let users = Users::new(db.clone());
    let books = Books::new(db.clone());
    let pages = Pages::new(db.clone());

    {
        let users = vec![
            User::new("User 1".to_string()),
            User::new("User 2".to_string()),
            User::new("User 3".to_string()),
        ];
        for user in users {
            User::create()
                .id(user.id)
                .name(user.name)
                .exec(&mut db)
                .await?;
        }
    }

    {
        use crate::data::Book;

        let users = users.find_all().await?;
        let user_id = users[0].id.clone();
        let books = vec![
            Book::new("こくご".to_string(), user_id.clone()),
            Book::new("さんすう".to_string(), user_id.clone()),
            Book::new("えいご".to_string(), user_id.clone()),
        ];
        for book in books {
            Book::create()
                .id(book.id)
                .name(book.name)
                .user_id(book.user_id)
                .exec(&mut db)
                .await?;
        }
    }

    {
        use crate::data::Page;
        let books_vec = books.find_all().await?;

        for book in books_vec {
            for page_name in 1..=30 {
                let page = Page::new(book.id.clone(), page_name.to_string());
                Page::create()
                    .book_id(page.book_id)
                    .id(page.id)
                    .name(page.name)
                    .exec(&mut db)
                    .await?;
            }
        }
    }

    {
        use crate::data::Report;

        let reports = [
            {
                let book_id = books.find_all().await?[0].id.clone();
                let book_pages = pages.find_by_book_id(&book_id).await?;
                let page_id = book_pages[1].id.clone();
                Report::new(book_id, "2026-10-01".to_string(), page_id)
            },
            {
                let book_id = books.find_all().await?[1].id.clone();
                let book_pages = pages.find_by_book_id(&book_id).await?;
                let page_id = book_pages[2].id.clone();
                Report::new(book_id, "2026-10-01".to_string(), page_id)
            },
            {
                let book_id = books.find_all().await?[1].id.clone();
                let book_pages = pages.find_by_book_id(&book_id).await?;
                let page_id = book_pages[3].id.clone();
                Report::new(book_id, "2026-10-02".to_string(), page_id)
            },
        ];
        for report in reports {
            Report::create()
                .id(report.id)
                .book_id(report.book_id)
                .date(report.date)
                .page_id(report.page_id)
                .exec(&mut db)
                .await?;
        }
    }

    Ok(())
}

#[derive(::clap::Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(::clap::Subcommand)]
enum Command {
    CreateBook(CreateBookArgs),
    CreatePage(CreatePageArgs),
    CreateUser(CreateUserArgs),
    Migration(::toasty_cli::MigrationCommand),
    Serve,
}

#[derive(::clap::Args)]
struct CreateBookArgs {
    #[arg(long)]
    name: String,
    #[arg(long)]
    user_id: String,
}

#[derive(::clap::Args)]
struct CreatePageArgs {
    #[arg(long)]
    book_id: String,
    #[arg(long)]
    name: String,
}

#[derive(::clap::Args)]
struct CreateUserArgs {
    #[arg(long)]
    name: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let cli = <Cli as ::clap::Parser>::parse();

    let mut db = crate::setup_db().await?;
    match cli.command.unwrap_or(Command::Serve) {
        Command::CreateBook(CreateBookArgs { name, user_id }) => {
            crate::data::Book::create()
                .id(::uuid::Uuid::new_v4().to_string())
                .name(name)
                .user_id(user_id)
                .exec(&mut db)
                .await?;
        }
        Command::CreatePage(CreatePageArgs { book_id, name }) => {
            crate::data::Page::create()
                .book_id(book_id)
                .id(::uuid::Uuid::new_v4().to_string())
                .name(name)
                .exec(&mut db)
                .await?;
        }
        Command::CreateUser(CreateUserArgs { name }) => {
            crate::data::User::create()
                .id(::uuid::Uuid::new_v4().to_string())
                .name(name)
                .exec(&mut db)
                .await?;
        }
        Command::Migration(_) => {
            let args = std::env::args();
            let db = crate::setup_db().await?;
            let toasty_cli = ::toasty_cli::ToastyCli::new(db);
            toasty_cli.parse_from(args).await?;
        }
        Command::Serve => {
            let app = crate::app::App::new(db).await?;
            let router_service = crate::app::router(app)?;
            ::topcoat::start(router_service).await?;
        }
    }
    Ok(())
}
