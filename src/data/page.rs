#[derive(::toasty::Model)]
#[key(id)]
pub struct Page {
    pub book_id: String,
    pub id: String,
    pub name: String,
    pub position: u16,
}

impl Page {
    pub fn new(book_id: String, name: String, position: u16) -> Self {
        Self {
            book_id,
            id: ::uuid::Uuid::new_v4().to_string(),
            name,
            position,
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
        Ok(Page::filter_by_id(id)
            .exec(&mut db)
            .await?
            .into_iter()
            .next())
    }
}
