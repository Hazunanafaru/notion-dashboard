use std::env;

/// Struct Config for setup environment variables
#[derive(PartialEq, Debug)]
pub struct Config {
    pub postgres_url: String,
    pub postgres_port: String,
    pub postgres_user: String,
    pub postgres_password: String,
    pub postgres_db: String,
    pub notion_token: String,
    pub notion_dj_database_id: String,
    pub notion_dj_dashboard_h1_id: String,
}

impl Default for Config {
    fn default() -> Self {
        let postgres_url: String = "localhost".to_string();
        let postgres_port: String = "5432".to_string();
        let postgres_user: String = "user".to_string();
        let postgres_password: String = "".to_string();
        let postgres_db: String = "notion_dashboard".to_string();
        let notion_token: String = "".to_string();
        let notion_dj_database_id: String = "".to_string();
        let notion_dj_dashboard_h1_id: String = "".to_string();

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

impl Config {
    pub fn from_envar() -> Self {
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_default() {
        let postgresql_url = "localhost".to_string();
        let postgresql_port = "5432".to_string();
        let postgresql_user = "user".to_string();
        let postgresql_password = "".to_string();
        let postgresql_db = "notion_dashboard".to_string();
        let notion_token = "".to_string();
        let notion_dj_database_id = "".to_string();
        let notion_dj_dashboard_h1_id = "".to_string();

        let result = Config::default();

        assert_eq!(result.postgres_url, postgresql_url);
        assert_eq!(result.postgres_port, postgresql_port);
        assert_eq!(result.postgres_user, postgresql_user);
        assert_eq!(result.postgres_password, postgresql_password);
        assert_eq!(result.postgres_db, postgresql_db);
        assert_eq!(result.notion_token, notion_token);
        assert_eq!(result.notion_dj_database_id, notion_dj_database_id);
        assert_eq!(result.notion_dj_dashboard_h1_id, notion_dj_dashboard_h1_id);
    }

    #[test]
    fn test_from_envar() {
        let postgresql_url = "localhost";
        let postgresql_port = "5432";
        let postgresql_user = "user";
        let postgresql_password = "hq812asd][;]";
        let postgresql_db = "notion_dashboard";
        let notion_token = "secret_1234567lkyqnsi_";
        let notion_dj_database_id = "adjoas8d12hduayufg81g2312asd";
        let notion_dj_dashboard_h1_id = "asdh1h298g9rwueh7139husfhew";

        env::set_var("POSTGRES_URL", postgresql_url);
        env::set_var("POSTGRES_PORT", postgresql_port);
        env::set_var("POSTGRES_USER", postgresql_user);
        env::set_var("POSTGRES_PASSWORD", postgresql_password);
        env::set_var("POSTGRES_DB", postgresql_db);
        env::set_var("NOTION_TOKEN", notion_token);
        env::set_var("NOTION_DJ_DATABASE_ID", notion_dj_database_id);
        env::set_var("NOTION_DJ_DASHBOARD_H1_ID", notion_dj_dashboard_h1_id);

        let result = Config::from_envar();

        assert_eq!(result.postgres_url, postgresql_url);
        assert_eq!(result.postgres_port, postgresql_port);
        assert_eq!(result.postgres_user, postgresql_user);
        assert_eq!(result.postgres_password, postgresql_password);
        assert_eq!(result.postgres_db, postgresql_db);
        assert_eq!(result.notion_token, notion_token);
        assert_eq!(result.notion_dj_database_id, notion_dj_database_id);
        assert_eq!(result.notion_dj_dashboard_h1_id, notion_dj_dashboard_h1_id);
    }
}
