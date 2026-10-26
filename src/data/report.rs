#[derive(::toasty::Model)]
#[key(id)]
pub struct Report {
    pub book_id: String,
    pub date: String,
    pub id: String,
    pub page_id: String,
}

impl Report {
    pub fn new(book_id: String, date: String, page_id: String) -> Self {
        Self {
            book_id,
            date,
            id: ::uuid::Uuid::new_v4().to_string(),
            page_id,
        }
    }
}

pub struct Reports {
    db: ::toasty::Db,
}

impl Reports {
    pub fn new(db: ::toasty::Db) -> Self {
        Self { db }
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
