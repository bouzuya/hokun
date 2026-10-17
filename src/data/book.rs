#[derive(::toasty::Model)]
#[key(id)]
pub struct Book {
    pub id: String,
    pub name: String,
}

pub struct Books {
    db: ::toasty::Db,
}

impl Books {
    pub async fn dummy(mut db: ::toasty::Db) -> Result<Self, ::toasty::Error> {
        let books = vec![
            Book {
                id: uuid::Uuid::new_v4().to_string(),
                name: "こくご".to_string(),
            },
            Book {
                id: uuid::Uuid::new_v4().to_string(),
                name: "さんすう".to_string(),
            },
            Book {
                id: uuid::Uuid::new_v4().to_string(),
                name: "えいご".to_string(),
            },
        ];
        for book in books {
            Book::create()
                .id(book.id)
                .name(book.name)
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
