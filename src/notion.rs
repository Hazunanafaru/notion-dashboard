use crate::model::*;
use reqwest::Client;
use serde_json::json;

const NOTION_API_ENDPOINT: &str = "https://api.notion.com/v1";
const NOTION_VERSION: &str = "2022-06-28";

/// Get Page Data from Notion
pub async fn get_page_data(token: &str, ndb_id: &str, date: &str) -> Result<PageData, String> {
    let client = Client::new();
    let endpoint = format!("{}/databases/{}/query", NOTION_API_ENDPOINT, ndb_id);
    let formated_date = format!("{}T00:00:00+07:00", date);
    let filter = serde_json::json!({"filter":{"timestamp":"created_time","created_time":{"after":formated_date}}});

    let resp = client
        .post(endpoint)
        .header("Notion-Version", NOTION_VERSION)
        .header("Content-Type", "application/json")
        .bearer_auth(token)
        .json(&filter)
        .send()
        .await
        .expect("Failed to get response")
        .json::<PageDatas>()
        .await
        .expect("Failed to get payload");
    match resp.results.first() {
        Some(page_data) => {
            println!("Page ID Daily Journal Database is {}", &page_data.id);
            Ok(page_data.into())
        }
        None => {
            let msg = format!("No Daily Journal created at {}", date);
            println!("{}", &msg);
            Err(msg)
        }
    }
}

/// Update Daily Journal Dashboard in Notino
pub async fn patch_daily_dashboard(token: &str, block_id: &str, total_dj_pages: i32) {
    let client = Client::new();
    let endpoint = format!("{}/blocks/{}", NOTION_API_ENDPOINT, block_id);
    let body = json!({
        "heading_1": {
            "rich_text": [
                {
                    "text": {
                        "content": total_dj_pages.to_string()
                    }
                }
            ]
        }
    });

    let resp = client
        .patch(endpoint)
        .header("Notion-Version", NOTION_VERSION)
        .bearer_auth(token)
        .json(&body)
        .send()
        .await
        .expect("Failed to patch object")
        .json::<PageData>()
        .await
        .expect("Failed to get payload");
    println!("Object ID Daily Journal Dashboard {}", &resp.id);
}
