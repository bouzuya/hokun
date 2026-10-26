#[derive(Clone, ::toasty::Model)]
#[key(id)]
pub struct User {
    pub id: String,
    pub name: String,
}

impl User {
    pub fn new(name: String) -> Self {
        Self {
            id: ::uuid::Uuid::new_v4().to_string(),
            name,
        }
    }
}

pub struct Users {
    db: ::toasty::Db,
}

impl Users {
    pub fn new(db: ::toasty::Db) -> Self {
        Self { db }
    }

    pub async fn find_all(&self) -> Result<Vec<User>, ::toasty::Error> {
        let mut db = self.db.clone();
        User::all().exec(&mut db).await
    }

    pub async fn find_by_id(&self, id: &str) -> Result<Option<User>, ::toasty::Error> {
        let mut db = self.db.clone();
        Ok(User::filter_by_id(id)
            .exec(&mut db)
            .await?
            .into_iter()
            .next())
    }
}
