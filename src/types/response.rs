use super::task::Task;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub items: Vec<HashMap<String, ItemWrapper>>,
    #[serde(rename = "current-item-index")]
    pub current_item_index: u32,
    pub schema: u32,
    #[serde(rename = "start-total-content-size")]
    pub start_total_content_size: u32,
    #[serde(rename = "end-total-content-size")]
    pub end_total_content_size: u32,
    #[serde(rename = "latest-total-content-size")]
    pub latest_total_content_size: u32,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ItemWrapper {
    #[serde(rename = "p")]
    pub item: Option<Task>,
    #[serde(rename = "e")]
    pub entity: String,
    #[serde(rename = "t")]
    // 0 = inser, 1 = edit, 2 = delete
    pub operation_type: u8,
}
