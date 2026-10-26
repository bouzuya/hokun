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
