use crate::types;
use serde::{Deserialize, Serialize};

table! {
    tasks (uuid) {
        uuid -> Text,
        title -> Varchar,
        trashed -> Nullable<Bool>,
        start -> Nullable<Integer>,
        status -> Nullable<Integer>,
        #[sql_name = "type"]
        type_project -> Nullable<Integer>,
        #[sql_name = "todayIndexReferenceDate"]
        today_index_reference_date -> Nullable<Double>,
        #[sql_name = "todayIndex"]
        today_index -> Nullable<Integer>,
        index -> Nullable<Integer>,
        #[sql_name = "startBucket"]
        start_bucket -> Nullable<Integer>,
        #[sql_name = "stopDate"]
        completion_date -> Nullable<Double>,

    }
}

#[derive(
    Clone, Insertable, Debug, Deserialize, Serialize, Queryable, Identifiable, AsChangeset,
)]
#[diesel(treat_none_as_null = true)]
#[diesel(primary_key(uuid))]
#[diesel(table_name = tasks)]
pub struct Task {
    pub uuid: String,
    pub title: String,
    pub trashed: Option<bool>,
    pub start: Option<i32>,  // 0 today, 1 Anytime, 2 Someday
    pub status: Option<i32>, // Started = 0, Pending = 1, Canceled = 2, Completed = 3,

    pub type_project: Option<i32>, // 0 = task, 1 = project, 2 = actiongroup
    pub today_index_reference_date: Option<f64>,
    pub today_index: Option<i32>,
    pub index: Option<i32>,
    pub start_bucket: Option<i32>,
    pub completion_date: Option<f64>, // Timestamp
}

impl From<types::task::Task> for Task {
    fn from(value: types::task::Task) -> Self {
        Self {
            uuid: value
                .uuid
                .clone()
                .expect("Creating a new task without a uuid"),
            title: value
                .title
                .unwrap_or(Some("".to_string()))
                .unwrap_or("".to_string()),
            trashed: value.trashed.unwrap_or_default(),
            start: value.start.unwrap_or_default(),
            status: value.status.unwrap_or_default(),

            type_project: value.type_project.unwrap_or_default(),
            today_index_reference_date: value.today_index_reference_date.unwrap_or(None),
            today_index: value.today_index.unwrap_or_default(),
            index: value.index.unwrap_or_default(),
            start_bucket: value.start_bucket.unwrap_or_default(),
            completion_date: value.completion_date.unwrap_or_default(),
        }
    }
}

impl Task {
    pub fn update_from(&mut self, value: types::task::Task) {
        self.uuid = value.uuid.or(Some(self.uuid.clone())).unwrap();
        self.title = value
            .title
            .or(Some(Some(self.title.clone())))
            .expect("title is not updated to null1")
            .expect("title is not updated to null2");
        self.trashed = value.trashed.or(Some(self.trashed.clone())).unwrap();
        self.start = value.start.or(Some(self.start.clone())).unwrap();
        self.status = value.status.or(Some(self.status.clone())).unwrap();

        self.type_project = value
            .type_project
            .or(Some(self.type_project.clone()))
            .unwrap();
        self.today_index_reference_date = value
            .today_index_reference_date
            .or(Some(self.today_index_reference_date.clone()))
            .unwrap();
        self.today_index = value
            .today_index
            .or(Some(self.today_index.clone()))
            .unwrap();
        self.index = value.index.or(Some(self.index.clone())).unwrap();
        self.start_bucket = value
            .start_bucket
            .or(Some(self.start_bucket.clone()))
            .unwrap();
        self.completion_date = value
            .completion_date
            .or(Some(self.completion_date.clone()))
            .unwrap();
    }
}
