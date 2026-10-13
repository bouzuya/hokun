use crate::data::{Books, Pages};

pub struct Reports(std::sync::Arc<std::sync::Mutex<Vec<Report>>>);

impl Reports {
    pub fn new() -> Self {
        Self(std::sync::Arc::new(std::sync::Mutex::new(Vec::new())))
    }

    pub fn dummy(books: &Books, pages: &Pages) -> Self {
        let reports = Self::new();
        {
            let book_id = books.find_all()[0].id.clone();
            let book_pages = pages.find_by_book_id(&book_id);
            let page_id = book_pages[1].id.clone();
            reports.create(Report {
                id: uuid::Uuid::new_v4().to_string(),
                book_id,
                date: "2026-10-01".to_string(),
                page_id,
            });
        }
        {
            let book_id = books.find_all()[1].id.clone();
            let book_pages = pages.find_by_book_id(&book_id);
            let page_id = book_pages[2].id.clone();
            reports.create(Report {
                id: uuid::Uuid::new_v4().to_string(),
                book_id,
                date: "2026-10-01".to_string(),
                page_id,
            });
        }
        {
            let book_id = books.find_all()[1].id.clone();
            let book_pages = pages.find_by_book_id(&book_id);
            let page_id = book_pages[3].id.clone();
            reports.create(Report {
                id: uuid::Uuid::new_v4().to_string(),
                book_id,
                date: "2026-10-02".to_string(),
                page_id,
            });
        }
        reports
    }

    pub fn create(&self, report: Report) {
        self.0.lock().unwrap().push(report);
    }

    pub fn find_by_date(&self, date: &str) -> Vec<Report> {
        self.0
            .lock()
            .unwrap()
            .iter()
            .cloned()
            .filter(|r| r.date == date)
            .collect()
    }

    pub fn find_by_id(&self, id: &str) -> Option<Report> {
        self.0.lock().unwrap().iter().cloned().find(|r| r.id == id)
    }

    pub fn find_by_month(&self, month: String) -> Vec<Report> {
        self.0
            .lock()
            .unwrap()
            .iter()
            .cloned()
            .filter(|r| r.date.starts_with(&month))
            .collect()
    }

    pub fn update(&self, report: Report) {
        let mut reports = self.0.lock().unwrap();
        if let Some(existing) = reports.iter_mut().find(|r| r.id == report.id) {
            *existing = report;
        }
    }
}

#[derive(Clone)]
pub struct Report {
    pub book_id: String,
    pub date: String,
    pub id: String,
    pub page_id: String,
}
