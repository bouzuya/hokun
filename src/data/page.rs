#[derive(::toasty::Model)]
#[key(id)]
pub struct Page {
    pub book_id: String,
    pub id: String,
    pub name: String,
}

impl Page {
    pub fn new(book_id: String, name: String) -> Self {
        Self {
            book_id,
            id: ::uuid::Uuid::new_v4().to_string(),
            name,
        }
    }
}

pub struct Pages {
    db: ::toasty::Db,
}

impl Pages {
    pub fn new(db: ::toasty::Db) -> Self {
        Self { db }
    }

    pub async fn find_by_book_id(&self, book_id: &str) -> Result<Vec<Page>, ::toasty::Error> {
        let mut db = self.db.clone();
        Page::filter(Page::fields().book_id().eq(book_id))
            .exec(&mut db)
            .await
    }

    pub async fn find_by_id(&self, id: &str) -> Result<Option<Page>, ::toasty::Error> {
        let mut db = self.db.clone();
        // FIXME: handle not found error
        Page::get_by_id(&mut db, id).await.map(Some)
    }
}
