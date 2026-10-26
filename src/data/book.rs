#[derive(::toasty::Model)]
#[key(id)]
pub struct Book {
    pub id: String,
    pub name: String,
    pub user_id: String,
}

impl Book {
    pub fn new(name: String, user_id: String) -> Self {
        Self {
            id: ::uuid::Uuid::new_v4().to_string(),
            name,
            user_id,
        }
    }
}

pub struct Books {
    db: ::toasty::Db,
}

impl Books {
    pub fn new(db: ::toasty::Db) -> Self {
        Self { db }
    }

    pub async fn find_all(&self) -> Result<Vec<Book>, ::toasty::Error> {
        let mut db = self.db.clone();
        Book::all().exec(&mut db).await
    }

    pub async fn find_by_id(&self, id: &str) -> Result<Option<Book>, ::toasty::Error> {
        let mut db = self.db.clone();
        // FIXME: handle not found error
        Book::get_by_id(&mut db, id).await.map(Some)
    }
}
