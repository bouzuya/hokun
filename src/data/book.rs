pub struct Books(Vec<Book>);

impl Books {
    pub fn dummy() -> Self {
        let books = vec![
            Book {
                id: "kokugo".to_string(),
                name: "こくご".to_string(),
            },
            Book {
                id: "sansuu".to_string(),
                name: "さんすう".to_string(),
            },
            Book {
                id: "eigo".to_string(),
                name: "えいご".to_string(),
            },
        ];
        Self(books)
    }

    pub fn find_all(&self) -> &Vec<Book> {
        &self.0
    }
}

pub struct Book {
    pub id: String,
    pub name: String,
}
