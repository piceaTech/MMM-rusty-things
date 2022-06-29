use crate::types;
use serde::{Serialize, Deserialize};

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
        #[sql_name = "stopDate"]
        completion_date -> Nullable<Double>,

    }
}




#[derive(Clone)]
#[derive(Insertable)]
#[derive(Debug)]
#[derive(Deserialize, Serialize)]
#[derive(Queryable, Identifiable, AsChangeset)]
#[changeset_options(treat_none_as_null="true")]
#[primary_key(uuid)]
#[table_name="tasks"]
pub struct Task{
  pub uuid: String,
  pub title: String,
  pub trashed: Option<bool>,
  pub start: Option<i32>, // 0 today, 1 Anytime, 2 Someday
  pub status: Option<i32>,// Started = 0, Pending = 1, Canceled = 2, Completed = 3,

  pub type_project: Option<i32>, // 0 = task, 1 = project, 2 = actiongroup
  pub today_index_reference_date: Option<f64>,
  pub today_index: Option<i32>,
  pub index: Option<i32>,
  pub completion_date: Option<f64>, // Timestamp
}

impl From<types::task::Task> for Task {
  fn from(value: types::task::Task) -> Self{
    
    Self{
      uuid: value.uuid.clone().expect("Creating a new task without a uuid"),
      title: value.title.expect(&format!("Creating a new task without a title {:?}", value.uuid.clone())).expect(&format!("Creating a new task without a title, 2 stage {:?}", value.uuid.clone())),
      trashed: value.trashed.expect("Creating a new task without trashed"),
      start: value.start.expect("Creating a new task without start"),
      status: value.status.expect("Creating a new task without status"),

      type_project: value.type_project.expect("Creating a new task without type_project"),
      today_index_reference_date: value.today_index_reference_date.unwrap_or(None),
      today_index: value.today_index.expect("Creating a new task without today_index"),
      index: value.index.expect("Creating a new task without today_index"),
      completion_date: value.completion_date.expect("Creating a new task without completion_date"),
    }

    
  }
}

impl Task {
  pub fn update_from(&mut self, value: types::task::Task){
    self.uuid = value.uuid.or(Some(self.uuid.clone())).unwrap();
    self.title = value.title.or(Some(Some(self.title.clone()))).expect("title is not updated to null1").expect("title is not updated to null2");
    self.trashed = value.trashed.or(Some(self.trashed.clone())).unwrap();
    self.start = value.start.or(Some(self.start.clone())).unwrap();
    self.status = value.status.or(Some(self.status.clone())).unwrap();

    self.type_project = value.type_project.or(Some(self.type_project.clone())).unwrap();
    self.today_index_reference_date = value.today_index_reference_date.or(Some(self.today_index_reference_date.clone())).unwrap();
    self.today_index = value.today_index.or(Some(self.today_index.clone())).unwrap();
    self.index = value.index.or(Some(self.index.clone())).unwrap();
    self.completion_date = value.completion_date.or(Some(self.completion_date.clone())).unwrap();
  }
}