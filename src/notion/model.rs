use serde::{Deserialize, Serialize};

/// NotionPropType Enum
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub enum NotionPropType {
    #[serde(rename = "unique_id")]
    UniqueIDType,
    #[serde(rename = "select")]
    SelectType,
    #[serde(rename = "multi_select")]
    MultiSelectType,
    #[serde(rename = "number")]
    NumberType,
    #[serde(rename = "title")]
    TitleType,
    #[serde(rename = "text")]
    #[default]
    TextType,
}

/// NotionBlockType Enum
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub enum NotionBlockType {
    #[serde(rename = "heading_1")]
    #[default]
    Heading1Type,
    #[serde(rename = "text")]
    TextType,
}

#[derive(Serialize, Deserialize, Debug, Default)]
/// DailyJournal hold data for each Daily Journal Created
pub struct DailyJournal {
    pub id: String,
    pub created_time: String,
    pub properties: Properties,
}

/// Properties struct and it's implementation
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Properties {
    #[serde(rename = "ID")]
    pub id: ID,
    #[serde(rename = "Verdict")]
    pub verdict: Verdict,
    #[serde(rename = "Tags")]
    pub tags: Tags,
    #[serde(rename = "kCal")]
    pub k_cal: KCal,
    #[serde(rename = "Name")]
    pub name: Name,
}

/// ID struct and it's implementation
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ID {
    pub r#type: NotionPropType,
    pub unique_id: UniqueId,
}

/// UniqueId struct and it's implementation
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct UniqueId {
    pub prefix: String,
    pub number: i32,
}

/// Verdict struct and it's implementation
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Verdict {
    pub r#type: NotionPropType,
    pub select: Select,
}

/// Select struct and it's implementation
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Select {
    pub id: String,
    pub name: String,
    pub color: String,
}

/// Tags struct and it's implementation
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Tags {
    pub r#type: NotionPropType,
    pub multi_select: Vec<Select>,
}

/// KCal (kCal) struct and it's implementation
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct KCal {
    pub r#type: NotionPropType,
    pub number: i32,
}

/// Name struct and it's implementation
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Name {
    pub r#type: NotionPropType,
    pub title: Vec<Text>,
}

/// Text struct and it's implementation
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Text {
    pub r#type: NotionPropType,
    pub plain_text: String,
}

/// Create new DailyJournal from borrowed DailyJournal
impl From<&DailyJournal> for DailyJournal {
    fn from(page_data: &DailyJournal) -> Self {
        DailyJournal {
            id: page_data.id.clone(),
            created_time: page_data.created_time.clone(),
            properties: page_data.properties.clone(),
        }
    }
}

/// Vector of DailyJournal
#[derive(Serialize, Deserialize, Debug)]
pub struct DailyJournals {
    pub results: Vec<DailyJournal>,
}

/// Heading 1 Block Struct and it's implementation
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct BlockHeading1 {
    pub id: String,
    pub created_time: String,
    pub r#type: NotionBlockType,
    pub heading_1: Heading1,
}

/// Heading 1 Struct and it's implementation
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Heading1 {
    pub rich_text: Vec<RichText>,
    pub is_toggleable: bool,
    pub color: String,
}

/// RichText Struct and it's implementation
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct RichText {
    pub r#type: NotionBlockType,
    pub plain_text: String,
}
