use crate::data::Users;

#[derive(::toasty::Model)]
#[key(id)]
pub struct Book {
    pub id: String,
    pub name: String,
    pub user_id: String,
}

pub struct Books {
    db: ::toasty::Db,
}

impl Books {
    pub async fn dummy(mut db: ::toasty::Db, users: &Users) -> Result<Self, ::toasty::Error> {
        let users = users.find_all().await?;
        let user_id = users[0].id.clone();
        let books = vec![
            Book {
                id: uuid::Uuid::new_v4().to_string(),
                name: "こくご".to_string(),
                user_id: user_id.clone(),
            },
            Book {
                id: uuid::Uuid::new_v4().to_string(),
                name: "さんすう".to_string(),
                user_id: user_id.clone(),
            },
            Book {
                id: uuid::Uuid::new_v4().to_string(),
                name: "えいご".to_string(),
                user_id: user_id.clone(),
            },
        ];
        for book in books {
            Book::create()
                .id(book.id)
                .name(book.name)
                .user_id(book.user_id)
                .exec(&mut db)
                .await?;
        }
        Ok(Self { db })
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
