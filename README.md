# Notion Dashboard

Simple job to update my internal notion dashboard.

## Requirement
### Job
1. rust
2. docker
3. sqlx
4. terraform (tfenv)
5. terragrunt (tgenv)
6. gcloud
7. Taskfile

## Development

1. Create `.compose.env` file and export it as environment variables
   ```
   cat << EOT > .compose.env
   POSTGRES_URL="db"
   POSTGRES_PORT="5432"
   POSTGRES_USER="<your_db_name>"
   POSTGRES_PASSWORD="<your_db_password>"
   POSTGRES_DB="notion_dashboard"
   NOTION_TOKEN="<your_notion_token>"
   NOTION_DJ_DATABASE_ID="<your_notion_dj_database_id>"
   NOTION_DJ_DASHBOARD_H1_ID="<your_notion_dj_dashboard_h1_id>"
   DATABASE_URL="postgresql://<your_db_name>:<your_db_password>@localhost:5432/notion_dashboard"
   EOT
   export $(xargs < .compose.env)
   ```
2. Build a Docker image with this command
   ```
   task image-build
   ```
3. Run docker compose
   ```
   task compose-up
   ```
4. Migrate database schema
   ```
   task migrate
   ```
5. Teardown docker compose and delete image
   ```
   task compose-down && task image-remove
   ```
