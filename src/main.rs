use chrono::offset::Local;
use notion_dashboard::{config::Envars, db::*, notion::*};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set date
    let date = Local::now().date_naive().to_string();
    println!("Start Daily Journal Dashboard for {}", date);

    // Load config
    let envars = Envars::default();

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

    // Initiate Migration
    run_migration(&pool).await;

    // Get page_data of today Daily Journal
    let page_data = match get_page_data(
        envars.notion_token.as_str(),
        envars.notion_dj_database_id.as_str(),
        &date,
    )
    .await
    {
        Ok(page_data) => page_data,
        Err(_) => {
            println!("Graceful shutdown due to no Daily Journal page today.");
            std::process::exit(0)
        }
    };

    // Insert Today page_id to notion_dashboard postgress database
    insert_page_id(&pool, &page_data.id, &date).await;

    // Aggregate Total page_id of this month
    let aggregate_record = update_monthly_data(&pool, &date).await?;

    // Patch Daily Journal Dashboard Total Pages of this month page
    patch_daily_dashboard(
        envars.notion_token.as_str(),
        envars.notion_dj_dashboard_h1_id.as_str(),
        aggregate_record.total_dj_pages,
    )
    .await;
    Ok(())
}
