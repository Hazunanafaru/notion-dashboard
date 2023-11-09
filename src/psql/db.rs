use crate::notion::model::DailyJournals;
use crate::psql::model::{Column, DailyJournal};
use log::{error, info};
use sqlx::migrate::Migrator;
use sqlx::{types::chrono::NaiveDate, Pool, Postgres};

static MIGRATOR: Migrator = sqlx::migrate!();

// Check database schema.
pub async fn check_database_schema(pool: &Pool<Postgres>) -> Result<(), String> {
    let records = sqlx::query_as::<_, Column>(
        "SELECT column_name FROM information_schema.columns WHERE table_name in ('daily_journal_pages')",
    )
    .fetch_all(pool)
    .await
    .expect("Error while checking database schema. Will run migrator.");

    if records.is_empty() {
        return Err("Database schema is empty. Will run migrator.".to_string());
    }

    records
        .iter()
        .for_each(|column| DailyJournal::schema_check(column));
    info!("Database Schema is compatible");
    Ok(())
}

/// Run Migrator to start database migration
pub async fn run_migration(pool: &Pool<Postgres>) {
    match MIGRATOR.run(pool).await {
        Ok(_) => info!("Migration succeed!"),
        Err(err) => {
            error!("Migration Error: {}. Exited Program.", err);
            std::process::exit(1)
        }
    };
}

/// Get this month Daily Journal count from daily_journal_pages table in PostgreSQL
pub async fn get_monthly_daily_journal_count(
    pool: &Pool<Postgres>,
    today: &str,
    first_day: &str,
) -> Result<i64, String> {
    let first_day_fmt_date =
        NaiveDate::parse_from_str(&first_day, "%Y-%m-%d").expect("Failed to parse first_day_date.");
    let today_fmt_date =
        NaiveDate::parse_from_str(&today, "%Y-%m-%d").expect("Failed to parse today_date (date).");
    let get_count: (i64,) =
        sqlx::query_as("SELECT COUNT(id) FROM daily_journal_pages WHERE date >= $1 AND date <= $2")
            .bind(first_day_fmt_date)
            .bind(today_fmt_date)
            .fetch_one(pool)
            .await
            .expect("Failed to get monthly Daily Journal count.");
    Ok(get_count.0)
}

/// Insert Page ID from notion to daily_journal_pages table in PostgreSQL
pub async fn insert_daily_journal(pool: &Pool<Postgres>, daily_journals: DailyJournals) {
    for dj in daily_journals.results {
        // Setup variables
        // DateTime format following date time format from notion
        // https://developers.notion.com/reference/page#date-property-values
        let formated_date = NaiveDate::parse_from_str(&dj.created_time, "%Y-%m-%dT%H:%M:%S.000Z")
            .expect("Failed to parse Date");
        let page_id = format!(
            "{}-{}",
            dj.properties.id.unique_id.prefix,
            dj.properties.id.unique_id.number.to_string()
        );
        let name = dj
            .properties
            .name
            .title
            .first()
            .unwrap()
            .plain_text
            .as_str();
        let verdict = dj.properties.verdict.select.name.as_str();
        let kcal = &dj.properties.k_cal.number;
        let mut tags = "".to_string();
        for tag in dj.properties.tags.multi_select {
            tags = tags + &tag.name
        }
        info!("Start to Insert Daily Journal {}", &formated_date);

        // Insert Daily Journal
        let record: Result<(i64,), sqlx::Error> =
            sqlx::query_as("INSERT INTO daily_journal_pages (date, page_id, name, verdict, kcal, tags) VALUES ($1, $2, $3, $4, $5, $6) RETURNING id")
                .bind(formated_date)
                .bind(page_id)
                .bind(name)
                .bind(verdict)
                .bind(kcal)
                // Convert tags to str in here since I don't know how to do that wihout change the
                // type
                .bind(tags.as_str())
                .fetch_one(pool)
                .await;
        match record {
            Ok((id,)) => info!("Daily Journal {} inserted with id {}", &formated_date, &id),
            Err(err) => error!(
                "Error during insertion of Daily Journal {}. {}",
                &formated_date, err
            ),
        }
    }
}
