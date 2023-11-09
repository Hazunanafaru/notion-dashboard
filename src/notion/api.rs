use crate::notion::model::{BlockHeading1, DailyJournals};
use log::{error, info};
use reqwest::Client;
use serde_json::json;

const NOTION_API_ENDPOINT: &str = "https://api.notion.com/v1";
const NOTION_VERSION: &str = "2022-06-28";

/// Get Daily Journal(s) from Notion
pub async fn get_daily_journals(
    token: &str,
    notion_db: &str,
    after: &str,
    before: &str,
) -> Result<DailyJournals, String> {
    info!(
        "Start to get Daily Journals Data from {} to {}.",
        &after, &before
    );

    // Setup
    let client = Client::new();
    let endpoint = format!("{}/databases/{}/query", NOTION_API_ENDPOINT, notion_db);
    let after_formated = format!("{}T00:00:00+07:00", after);
    let before_formated = format!("{}T00:00:00+07:00", before);
    let filter = serde_json::json!({
          "filter": {
            "and": [
              {
                "timestamp": "created_time",
                "created_time": {
                  "after": after_formated
                }
              },
              {
                "timestamp": "created_time",
                "created_time": {
                  "before": before_formated
                }
              }
            ]
          }
        }
    );

    // Sent request
    let resp = client
        .post(endpoint)
        .header("Notion-Version", NOTION_VERSION)
        .header("Content-Type", "application/json")
        .bearer_auth(token)
        .json(&filter)
        .send()
        .await
        .expect("Failed to get response")
        .json::<DailyJournals>()
        .await
        .expect("Failed to get payload");

    // Check every daily journals retrived
    resp.results.iter().for_each(|daily_journal| {
        info!(
            "Daily Journal at {} with id {} retrived.",
            &daily_journal.created_time, &daily_journal.id
        )
    });
    Ok(resp)
}

/// Update Daily Journal Written this Month Dashboard
/// in Notion
pub async fn patch_daily_dashboard(
    token: &str,
    block_id: &str,
    pages_count: i32,
) -> Result<(), String> {
    // Setup
    let client = Client::new();
    let endpoint = format!("{}/blocks/{}", NOTION_API_ENDPOINT, block_id);
    let body = json!({
        "heading_1": {
            "rich_text": [
                {
                    "text": {
                        "content": pages_count.to_string()
                    }
                }
            ]
        }
    });

    // Sent Request
    match client
        .patch(endpoint)
        .header("Notion-Version", NOTION_VERSION)
        .bearer_auth(token)
        .json(&body)
        .send()
        .await
        .expect("Failed to patch object")
        .json::<BlockHeading1>()
        .await
    {
        Ok(resp) => {
            info!(
                "Daily Journal Written this Month Dashboard value updated to {}",
                &resp.heading_1.rich_text.first().unwrap().plain_text
            );
            Ok(())
        }
        Err(err) => {
            error!("Failed to update Daily Journal Dashboard. {}", err);
            Err(err.to_string())
        }
    }
}
