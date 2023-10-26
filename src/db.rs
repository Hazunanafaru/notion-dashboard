use crate::model::*;
use sqlx::{types::chrono::NaiveDate, Pool, Postgres};

/// Insert Page ID from notion to pages table in PostgreSQL
pub async fn insert_page_id(pool: &Pool<Postgres>, page_id: &str, date: &str) {
    let formated_date = NaiveDate::parse_from_str(date, "%Y-%m-%d").expect("Failed to parse Date");
    let record: (i64,) =
        sqlx::query_as("INSERT INTO pages (date, page_id) VALUES ($1, $2) RETURNING id")
            .bind(formated_date)
            .bind(page_id)
            .fetch_one(pool)
            .await
            .expect("Failed to insert Daily Journal Page");
    println!("Insert ID is {}", &record.0);
}

/// Update Monthly Data on monthly_datas table or create new netry
pub async fn update_monthly_data(pool: &Pool<Postgres>, date: &str) -> Result<MonthlyData, String> {
    let splitted_date: Vec<&str> = date.split('-').collect();
    let year = splitted_date[0].parse::<i64>().unwrap();
    let month = splitted_date[1].parse::<i64>().unwrap();

    // Check if we have an entry for this month
    let get_record: Result<(i64, i32), sqlx::Error> = sqlx::query_as(
        "SELECT id, total_dj_pages FROM monthly_datas WHERE year = $1 AND month = $2",
    )
    .bind(year)
    .bind(month)
    .fetch_one(pool)
    .await;

    let mut result: MonthlyData = MonthlyData {
        id: PostgresId(0_i64),
        total_dj_pages: 0_i32,
    };

    // If there is no entry of this month, create new entry
    if get_record.is_err() {
        let record: (i64, i32) =sqlx::query_as("INSERT INTO monthly_datas (year, month, total_dj_pages) VALUES ($1, $2, $3) RETURNING id, total_dj_pages")
                .bind(year)
                .bind(month)
                .bind(1_i32)
                .fetch_one(pool)
                .await
                .expect("Failed to create monthly data");
        result.update_id(PostgresId(record.0));
        result.update_total_dj_pages(record.1);
    // Else update this month entry with increment of 1
    } else {
        let id = get_record.as_ref().unwrap().0;
        let total_dj_pages = get_record.as_ref().unwrap().1;
        let record: (i64,) = sqlx::query_as(
            "UPDATE monthly_datas SET total_dj_pages = $1 WHERE id = $2 RETURNING id",
        )
        .bind(total_dj_pages + 1)
        .bind(id)
        .fetch_one(pool)
        .await
        .expect("Failed to update monthly data");
        result.update_id(PostgresId(record.0));
        result.update_total_dj_pages(total_dj_pages + 1);
    }
    println!("Monthly Data with id {} was updated!", &result.id);
    println!(
        "Total Daily Journal this month is {}",
        &result.total_dj_pages
    );
    Ok(result)
}
