use chrono::{offset::Local, DateTime, Duration};
use env_logger;
use log::{debug, error, info};
use notion_dashboard::{config::config::Envars, notion::api::*, psql::db::*};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup logging
    env_logger::init();

    // Setup naive date to check the yesterday Daily Journal
    let today = Local::now();
    let yesteday = today.clone() - Duration::days(1);
    let today_string = today.date_naive().to_string();
    let yesterday_string = yesteday.date_naive().to_string();
    let splitted_date: Vec<&str> = today_string.split('-').collect();
    let first_day_string = format!("{}-{}-01", splitted_date[0], splitted_date[1]);
    info!(
        "Start Daily Journal Dashboard Cronjob for {}.",
        today.date_naive().to_string()
    );

    // Load config
    let envars = Envars::default();
    info!("Loading environment variables.");

    // Setup DB Connection
    let conn_url = format!(
        "postgress://{}:{}@{}:{}/{}",
        envars.postgres_user,
        envars.postgres_password,
        envars.postgres_url,
        envars.postgres_port,
        envars.postgres_db
    );
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(&conn_url)
        .await?;
    info!("Setup PostgreSQL connection.");

    // Check database schema. If there is no matched schema, run migration
    let _schema_check = match check_database_schema(&pool).await {
        Ok(_) => {}
        Err(_) => run_migration(&pool).await,
    };

    // Check this month Daily Journal count in postgres
    let mut psql_monthly_daily_journal_count = 0_i64;
    match get_monthly_daily_journal_count(&pool, &today_string, &first_day_string).await {
        Ok(count) => psql_monthly_daily_journal_count = count,
        Err(err) => {
            error!("{}", err)
        }
    }

    let daily_journals = match psql_monthly_daily_journal_count {
        0 => {
            // Get this month daily journals
            get_daily_journals(
                envars.notion_token.as_str(),
                envars.notion_dj_database_id.as_str(),
                &first_day_string,
                &today_string,
            )
            .await
            .expect("Failed to get this month daily journals")
        }
        _ => {
            // Get today daily journal
            get_daily_journals(
                envars.notion_token.as_str(),
                envars.notion_dj_database_id.as_str(),
                &yesterday_string,
                &today_string,
            )
            .await
            .expect("Failed to get today daily journal")
        }
    };
    let daily_journals_count =
        daily_journals.results.len() as i32 + psql_monthly_daily_journal_count as i32;

    // Insert Daily Journals to postgreSQL database
    insert_daily_journal(&pool, daily_journals).await;

    // Update Daily Journal Written this Month Notion Dashboard
    let _ = patch_daily_dashboard(
        envars.notion_token.as_str(),
        envars.notion_dj_dashboard_h1_id.as_str(),
        daily_journals_count,
    )
    .await;
    Ok(())
}
