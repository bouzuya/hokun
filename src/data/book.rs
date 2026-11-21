#[derive(Debug, ::toasty::Model)]
#[key(id)]
pub struct Book {
    pub id: String,
    pub name: String,
    pub user_id: String,
    pub position: u16,
}

impl Book {
    pub fn new(name: String, user_id: String, position: u16) -> Self {
        Self {
            id: ::uuid::Uuid::new_v4().to_string(),
            name,
            user_id,
            position,
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
        Ok(Book::filter_by_id(id)
            .exec(&mut db)
            .await?
            .into_iter()
            .next())
    }

    pub async fn filter_by_user_id(&self, user_id: &str) -> Result<Vec<Book>, ::toasty::Error> {
        let mut db = self.db.clone();
        Ok(Book::filter(Book::fields().user_id().eq(user_id))
            .order_by(Book::fields().position().asc())
            .exec(&mut db)
            .await?)
    }
}
