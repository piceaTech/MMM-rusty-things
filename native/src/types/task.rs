use serde_json::Value;
use std::convert::From;

use super::deserialize_some;
use serde::{Deserialize, Serialize};

impl From<Value> for Task {
    fn from(value: Value) -> Self {
        serde_json::from_value(value).unwrap()
    }
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct Task {
    #[serde(rename = "uuid")]
    pub uuid: Option<String>,
    #[serde(rename = "tt", default, deserialize_with = "deserialize_some")]
    pub title: Option<Option<String>>,
    #[serde(rename = "tr", default, deserialize_with = "deserialize_some")]
    pub trashed: Option<Option<bool>>,
    #[serde(rename = "st", default, deserialize_with = "deserialize_some")]
    pub start: Option<Option<i32>>, // Today = 0, Anytime = 1, Someday = 2,
    #[serde(rename = "ss", default, deserialize_with = "deserialize_some")]
    pub status: Option<Option<i32>>, // Started = 0, Pending = 1, Canceled = 2, Completed = 3,
    #[serde(rename = "tp", default, deserialize_with = "deserialize_some")]
    pub type_project: Option<Option<i32>>, // 0 = task, 1 = project, 2 = actiongroup
    #[serde(rename = "tir", default, deserialize_with = "deserialize_some")] // will be set to null
    pub today_index_reference_date: Option<Option<f64>>,
    #[serde(rename = "ti", default, deserialize_with = "deserialize_some")]
    pub today_index: Option<Option<i32>>,
    #[serde(rename = "sp", default, deserialize_with = "deserialize_some")] // will be set to null
    pub completion_date: Option<Option<f64>>, // Timestamp

    // fields after this are only there so we can detect changes to fields we are not interested in. Else we would delete it from the db even if we set some fields there.
    #[serde(rename = "pr", default, deserialize_with = "deserialize_some")]
    pub project_ids: Option<Option<Vec<String>>>,
    #[serde(rename = "dds", default, deserialize_with = "deserialize_some")]
    pub unknown17: Option<Option<i32>>,
    #[serde(rename = "icsd", default, deserialize_with = "deserialize_some")]
    pub unknown18: Option<Option<f64>>,
    #[serde(rename = "icp", default, deserialize_with = "deserialize_some")]
    pub unknown19: Option<Option<bool>>,

    #[serde(rename = "icc", default, deserialize_with = "deserialize_some")]
    pub unknown4: Option<Option<i32>>,
    #[serde(rename = "acrd", default, deserialize_with = "deserialize_some")]
    pub unknown5: Option<Option<f64>>,

    #[serde(rename = "do", default, deserialize_with = "deserialize_some")]
    pub unknown7: Option<Option<i32>>,
    #[serde(rename = "dl", default, deserialize_with = "deserialize_some")]
    pub unknown8: Option<Option<Vec<i32>>>,
    #[serde(rename = "lai", default, deserialize_with = "deserialize_some")]
    pub unknown9: Option<Option<f64>>,
    #[serde(rename = "ato", default, deserialize_with = "deserialize_some")]
    pub unknown10: Option<Option<i32>>,
    #[serde(rename = "sb", default, deserialize_with = "deserialize_some")]
    pub unknown11: Option<Option<i32>>,

    #[serde(rename = "ix", default, deserialize_with = "deserialize_some")]
    pub index: Option<Option<i32>>,
    #[serde(rename = "cd", default, deserialize_with = "deserialize_some")]
    pub creation_date: Option<Option<f64>>, // Timestamp
    #[serde(rename = "md", default, deserialize_with = "deserialize_some")]
    pub modification_date: Option<Option<f64>>, // Timestamp
    #[serde(rename = "sr", default, deserialize_with = "deserialize_some")]
    pub scheduled_date: Option<Option<f64>>, // Timestamp

    #[serde(rename = "dd", default, deserialize_with = "deserialize_some")]
    pub deadline_date: Option<Option<f64>>, // Timestamp

    #[serde(rename = "nt", default, deserialize_with = "deserialize_some")]
    pub note: Option<Option<NoteOptions>>,

    #[serde(rename = "ar", default, deserialize_with = "deserialize_some")]
    pub area_ids: Option<Option<Vec<String>>>,
    #[serde(rename = "pr", default, deserialize_with = "deserialize_some")]
    pub parent_task_ids: Option<Option<Vec<String>>>,
    #[serde(rename = "tg", default, deserialize_with = "deserialize_some")]
    pub tag_ids: Option<Option<Vec<String>>>,

    #[serde(rename = "rt", default, deserialize_with = "deserialize_some")]
    pub recurrence_task_ids: Option<Option<Vec<String>>>,

    #[serde(rename = "agr", default, deserialize_with = "deserialize_some")]
    pub action_group_ids: Option<Option<Vec<String>>>,
    #[serde(rename = "rr", default, deserialize_with = "deserialize_some")]
    pub repeater: Option<Option<RepeaterConfiguration3>>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct NoteStruct {
    #[serde(rename = "_t", default, deserialize_with = "deserialize_some")]
    pub tag: Option<Option<String>>,
    #[serde(rename = "ch", default, deserialize_with = "deserialize_some")]
    pub unknown20: Option<Option<i32>>,
    #[serde(rename = "t", default, deserialize_with = "deserialize_some")]
    pub unknown21: Option<Option<i32>>,
    #[serde(rename = "v", default, deserialize_with = "deserialize_some")]
    pub value: Option<Option<String>>,
    #[serde(rename = "ps", default, deserialize_with = "deserialize_some")]
    pub update: Option<Vec<NoteLine>>,
}
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct NoteLine {
    #[serde(rename = "p", default, deserialize_with = "deserialize_some")]
    pub unknown22: Option<Option<i32>>,
    #[serde(rename = "r", default, deserialize_with = "deserialize_some")]
    pub unknown23: Option<Option<String>>,
    #[serde(rename = "ch", default, deserialize_with = "deserialize_some")]
    pub change_time: Option<Option<i32>>,
    #[serde(rename = "l", default, deserialize_with = "deserialize_some")]
    pub line: Option<Option<i32>>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum NoteOptions {
    Str(String),
    Struc(NoteStruct),
}

impl Task {
    pub fn is_empty(&self) -> bool {
        self.project_ids.is_none()
            && self.unknown17.is_none()
            && self.unknown18.is_none()
            && self.unknown19.is_none()
            && self.today_index_reference_date.is_none()
            && self.unknown4.is_none()
            && self.unknown5.is_none()
            && self.today_index.is_none()
            && self.unknown7.is_none()
            && self.unknown8.is_none()
            && self.unknown9.is_none()
            && self.unknown10.is_none()
            && self.unknown11.is_none()
            && self.index.is_none()
            && self.creation_date.is_none()
            && self.modification_date.is_none()
            && self.scheduled_date.is_none()
            && self.completion_date.is_none()
            && self.deadline_date.is_none()
            && self.status.is_none()
            && self.type_project.is_none()
            && self.title.is_none()
            && self.note.is_none()
            && self.area_ids.is_none()
            && self.parent_task_ids.is_none()
            && self.tag_ids.is_none()
            && self.trashed.is_none()
            && self.recurrence_task_ids.is_none()
            && self.start.is_none()
            && self.action_group_ids.is_none()
            && self.repeater.is_none()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RepeaterConfiguration3 {
    #[serde(rename = "sr")]
    pub unknown12: f64,
    #[serde(rename = "ts")]
    pub unknown13: i32,
    #[serde(rename = "tp")]
    pub unknown14: f64,
    #[serde(rename = "rrv")]
    pub unknown15: i32,
    #[serde(rename = "ia")]
    pub first_scheduled_at: f64,
    #[serde(rename = "rc")]
    pub repeat_count: i32,
    #[serde(rename = "fu")]
    pub frequency_unit: u32,
    #[serde(rename = "fa")]
    pub frequency_amplitude: i32,
    #[serde(rename = "of")]
    pub detail_configuration: Vec<RepeaterDetailConfiguration>,
    #[serde(rename = "ed")]
    pub last_scheduled_at: Option<f64>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RepeaterDetailConfiguration {
    #[serde(rename = "dy", default, deserialize_with = "deserialize_some")]
    pub day: Option<i32>,
    #[serde(rename = "mo", default, deserialize_with = "deserialize_some")]
    pub month: Option<i32>,
    #[serde(rename = "wd", default, deserialize_with = "deserialize_some")]
    pub weekday: Option<u32>,
    #[serde(rename = "wdo", default, deserialize_with = "deserialize_some")]
    pub month_of: Option<i32>,
}
