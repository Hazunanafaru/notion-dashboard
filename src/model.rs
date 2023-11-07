use crate::db::Column;
use log::debug;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
/// DailyJournal hold data for each Daily Journal Created
pub struct DailyJournal {
    pub id: String,
    pub date: String,
    pub name: String,
    pub verdict: String,
    pub k_cal: i32,
    pub tags: String,
}

/// Create new DailyJournal from borrowed DailyJournal
impl From<&DailyJournal> for DailyJournal {
    fn from(page_data: &DailyJournal) -> Self {
        DailyJournal {
            id: page_data.id.clone(),
            date: page_data.date.clone(),
            name: page_data.name.clone(),
            verdict: page_data.verdict.clone(),
            k_cal: page_data.k_cal.clone(),
            tags: page_data.tags.clone(),
        }
    }
}

/// Create DailyJournal with default value
impl Default for DailyJournal {
    fn default() -> Self {
        DailyJournal {
            id: "".to_string(),
            date: "".to_string(),
            name: "".to_string(),
            verdict: "".to_string(),
            k_cal: 0,
            tags: "".to_string(),
        }
    }
}

/// Implementation of DailyJournal
impl DailyJournal {
    pub fn schema_check(col: &Column) {
        match col.column_name.as_str() {
            "id" | "page_id" | "date" | "name" | "verdict" | "kcal" | "tags" => {
                debug!(
                    "Column {} in database schema is detected",
                    col.column_name.clone()
                )
            }
            _ => {
                let msg = format!(
                    "Invalid column {} in database schema is detected",
                    col.column_name.clone()
                );
                debug!("{}", msg);
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
/// Vector of DailyJournal
pub struct DailyJournals {
    pub results: Vec<DailyJournal>,
}

// #[derive(Deserialize, Debug)]
// /// Custom type of Postgres ID
// pub struct PostgresId(pub i64);
//
// impl Display for PostgresId {
//     fn fmt(&self, f: &mut Formatter<'_>) -> Result {
//         write!(f, "{}", self.0)
//     }
// }
//
// #[derive(Deserialize, Debug)]
// /// Monthly Data that consist of id and total daily journl pages of this month
// pub struct MonthlyData {
//     pub id: PostgresId,
//     pub total_dj_pages: i32,
// }
//
// impl MonthlyData {
//     pub fn update_id(&mut self, id: PostgresId) {
//         self.id = id;
//     }
//
//     pub fn update_total_dj_pages(&mut self, total_dj_pages: i32) {
//         self.total_dj_pages = total_dj_pages;
//     }
// }
