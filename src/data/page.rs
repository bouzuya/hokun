use crate::data::Books;

pub struct Pages(Vec<Page>);

impl Pages {
    pub async fn dummy(books: &Books) -> Result<Self, ::toasty::Error> {
        let books_vec = books.find_all().await?;

        let pages = {
            let mut pages = vec![];
            for book in books_vec {
                for page_name in 1..=30 {
                    pages.push(Page {
                        book_id: book.id.clone(),
                        id: uuid::Uuid::new_v4().to_string(),
                        name: page_name.to_string(),
                    });
                }
            }
            pages
        };

        Ok(Self(pages))
    }

    pub fn find_by_book_id(&self, book_id: &str) -> Vec<&Page> {
        self.0.iter().filter(|it| it.book_id == book_id).collect()
    }

    pub fn find_by_id(&self, book_id: &str, page_id: &str) -> Option<&Page> {
        self.0
            .iter()
            .find(|it| it.book_id == book_id && it.id == page_id)
    }
}

pub struct Page {
    pub book_id: String,
    pub id: String,
    pub name: String,
}
