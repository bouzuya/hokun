use crate::data::Books;

#[derive(::toasty::Model)]
#[key(id)]
pub struct Page {
    pub book_id: String,
    pub id: String,
    pub name: String,
}

pub struct Pages {
    db: ::toasty::Db,
}

impl Pages {
    pub async fn dummy(mut db: ::toasty::Db, books: &Books) -> Result<Self, ::toasty::Error> {
        let books_vec = books.find_all().await?;

        for book in books_vec {
            for page_name in 1..=30 {
                Page::create()
                    .book_id(book.id.clone())
                    .id(uuid::Uuid::new_v4().to_string())
                    .name(page_name.to_string())
                    .exec(&mut db)
                    .await?;
            }
        }

        Ok(Self { db })
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
