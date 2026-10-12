pub struct Books(Vec<Book>);

impl Books {
    pub fn dummy() -> Self {
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
        Self(books)
    }

    pub fn find_all(&self) -> &Vec<Book> {
        &self.0
    }

    pub fn find_by_id(&self, id: &str) -> Option<&Book> {
        self.0.iter().find(|b| b.id == id)
    }
}

pub struct Book {
    pub id: String,
    pub name: String,
}
