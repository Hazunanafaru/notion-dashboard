use std::env;

/// Struct Envars for setup environment variables
pub struct Envars {
    pub postgres_url: String,
    pub postgres_port: String,
    pub postgres_user: String,
    pub postgres_password: String,
    pub postgres_db: String,
    pub notion_token: String,
    pub notion_dj_database_id: String,
    pub notion_dj_dashboard_h1_id: String,
}

impl Default for Envars {
    fn default() -> Self {
        let postgres_url: String =
            env::var("POSTGRES_URL").expect("Failed to load POSTGRES_URL environment variable.");
        let postgres_port: String =
            env::var("POSTGRES_PORT").expect("Failed to load POSTGRES_PORT environment variable.");
        let postgres_user: String =
            env::var("POSTGRES_USER").expect("Failed to load POSTGRES_USER environment variable.");
        let postgres_password: String = env::var("POSTGRES_PASSWORD")
            .expect("Failed to load POSTGRES_PASSWORD environment variable.");
        let postgres_db: String =
            env::var("POSTGRES_DB").expect("Failed to load POSTGRES_DB environment variable.");
        let notion_token: String =
            env::var("NOTION_TOKEN").expect("Failed to load NOTION_TOKEN environment variable.");
        let notion_dj_database_id: String = env::var("NOTION_DJ_DATABASE_ID")
            .expect("Failed to load NOTION_DJ_DATABASE_ID environment variable.");
        let notion_dj_dashboard_h1_id: String = env::var("NOTION_DJ_DASHBOARD_H1_ID")
            .expect("Failed to load NOTION_DJ_DASHBOARD_H1_ID environment variable.");

        Self {
            postgres_url,
            postgres_port,
            postgres_user,
            postgres_password,
            postgres_db,
            notion_token,
            notion_dj_database_id,
            notion_dj_dashboard_h1_id,
        }
    }
}
