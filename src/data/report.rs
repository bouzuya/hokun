use crate::data::{Books, Pages};

#[derive(::toasty::Model)]
#[key(id)]
pub struct Report {
    pub book_id: String,
    pub date: String,
    pub id: String,
    pub page_id: String,
}

pub struct Reports {
    db: ::toasty::Db,
}

impl Reports {
    pub async fn dummy(
        mut db: ::toasty::Db,
        books: &Books,
        pages: &Pages,
    ) -> Result<Self, ::toasty::Error> {
        {
            let book_id = books.find_all().await?[0].id.clone();
            let book_pages = pages.find_by_book_id(&book_id).await?;
            let page_id = book_pages[1].id.clone();
            Report::create()
                .id(uuid::Uuid::new_v4().to_string())
                .book_id(book_id)
                .date("2026-10-01".to_string())
                .page_id(page_id)
                .exec(&mut db)
                .await?;
        }
        {
            let book_id = books.find_all().await?[1].id.clone();
            let book_pages = pages.find_by_book_id(&book_id).await?;
            let page_id = book_pages[2].id.clone();
            Report::create()
                .id(uuid::Uuid::new_v4().to_string())
                .book_id(book_id.clone())
                .date("2026-10-01".to_string())
                .page_id(page_id.clone())
                .exec(&mut db)
                .await?;
        }
        {
            let book_id = books.find_all().await?[1].id.clone();
            let book_pages = pages.find_by_book_id(&book_id).await?;
            let page_id = book_pages[3].id.clone();
            Report::create()
                .id(uuid::Uuid::new_v4().to_string())
                .book_id(book_id)
                .date("2026-10-02".to_string())
                .page_id(page_id)
                .exec(&mut db)
                .await?;
        }

        Ok(Self { db })
    }

    pub async fn create(&self, report: Report) -> Result<(), ::toasty::Error> {
        let mut db = self.db.clone();
        Report::create()
            .id(report.id)
            .book_id(report.book_id)
            .date(report.date)
            .page_id(report.page_id)
            .exec(&mut db)
            .await?;
        Ok(())
    }

    pub async fn find_by_date(&self, date: &str) -> Result<Vec<Report>, ::toasty::Error> {
        let mut db = self.db.clone();
        Report::filter(Report::fields().date().eq(date))
            .exec(&mut db)
            .await
    }

    pub async fn find_by_id(&self, id: &str) -> Result<Option<Report>, ::toasty::Error> {
        let mut db = self.db.clone();
        Report::get_by_id(&mut db, id).await.map(Some)
    }

    pub async fn find_by_month(&self, month: String) -> Result<Vec<Report>, ::toasty::Error> {
        let mut db = self.db.clone();
        Report::filter(Report::fields().date().starts_with(month))
            .exec(&mut db)
            .await
    }

    pub async fn update(&self, mut report: Report) -> Result<(), ::toasty::Error> {
        let mut db = self.db.clone();
        report.update().exec(&mut db).await
    }
}
