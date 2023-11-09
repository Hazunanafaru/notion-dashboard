use log::debug;
use serde::{Deserialize, Serialize};

/// DailyJournal struct for postgresql and it's implementation
#[derive(Serialize, Deserialize, Debug)]
pub struct DailyJournal {
    id: i64,
    page_id: String,
    name: String,
    date: String,
    verdict: String,
    k_cal: i32,
    tags: String,
}

impl DailyJournal {
    /// Schema Checking for the postgresql
    /// Will loop for every column that should be available in
    /// Daily Journal table
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

// Column derived from sqlx FromRow
// To serialize column data when do Schema Checking
#[derive(sqlx::FromRow)]
pub struct Column {
    pub column_name: String,
}

/// Monthly Data that consist of id and total daily journl pages of this month
#[derive(Serialize, Deserialize, Debug)]
pub struct MonthlyData {
    pub id: i64,
    pub total_dj_pages: i32,
}

impl MonthlyData {
    pub fn update_id(&mut self, id: i64) {
        self.id = id;
    }

    pub fn update_total_dj_pages(&mut self, total_dj_pages: i32) {
        self.total_dj_pages = total_dj_pages;
    }
}
