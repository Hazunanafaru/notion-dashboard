use serde::Deserialize;
use std::fmt::{Display, Formatter, Result};

#[derive(Deserialize, Debug)]
/// PageData hold page id of today daily journal
pub struct PageData {
    pub id: String,
}

/// Create new PageData from borrowed PageData
impl From<&PageData> for PageData {
    fn from(page_data: &PageData) -> Self {
        PageData {
            id: page_data.id.clone(),
        }
    }
}

#[derive(Deserialize, Debug)]
/// Vector of PageData
pub struct PageDatas {
    pub results: Vec<PageData>,
}

#[derive(Deserialize, Debug)]
/// Custom type of Postgres ID
pub struct PostgresId(pub i64);

impl Display for PostgresId {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Deserialize, Debug)]
/// Monthly Data that consist of id and total daily journl pages of this month
pub struct MonthlyData {
    pub id: PostgresId,
    pub total_dj_pages: i32,
}

impl MonthlyData {
    pub fn update_id(&mut self, id: PostgresId) {
        self.id = id;
    }

    pub fn update_total_dj_pages(&mut self, total_dj_pages: i32) {
        self.total_dj_pages = total_dj_pages;
    }
}
