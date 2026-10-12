use crate::data::Books;

pub struct Pages(Vec<Page>);

impl Pages {
    pub fn dummy(books: &Books) -> Self {
        let books_vec = books.find_all();

        let pages = {
            let mut pages = vec![];
            for page_id in 1..=30 {
                pages.push(Page {
                    book_id: books_vec[0].id.clone(),
                    id: page_id.to_string(),
                    name: page_id.to_string(),
                });
            }
            for page_id in 1..=40 {
                pages.push(Page {
                    book_id: books_vec[1].id.clone(),
                    id: page_id.to_string(),
                    name: page_id.to_string(),
                });
            }
            pages
        };

        Self(pages)
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
