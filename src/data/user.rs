#[derive(::toasty::Model)]
#[key(id)]
pub struct User {
    pub id: String,
}

pub struct Users {
    db: ::toasty::Db,
}

impl Users {
    pub async fn dummy(mut db: ::toasty::Db) -> Result<Self, ::toasty::Error> {
        let users = vec![
            User {
                id: uuid::Uuid::new_v4().to_string(),
            },
            User {
                id: uuid::Uuid::new_v4().to_string(),
            },
            User {
                id: uuid::Uuid::new_v4().to_string(),
            },
        ];
        for user in users {
            User::create().id(user.id).exec(&mut db).await?;
        }
        Ok(Self { db })
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
