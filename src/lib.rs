#[macro_use]
extern crate neon;

#[macro_use]
extern crate neon_serde2 as neon_serde;

#[macro_use]
extern crate diesel;
use diesel::insert_into;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use diesel_migrations::EmbeddedMigrations;
use diesel_migrations::{embed_migrations, MigrationHarness};

extern crate dotenv;

use sql::meta::Meta;
use types::response::Response;
use types::task::Task;

use std::env;
use std::path::PathBuf;

use std::time::{Duration, SystemTime, UNIX_EPOCH};

mod sql;
mod types;

// #[no_mangle]
// pub extern "C" fn node_addon_init() {
//     __LOAD_NEON_MODULE();
// }
export! {
  fn getTodayEntries(dirname: String) -> Vec<sql::task::Task> {
    load_env(&dirname);
    get_today_tasks().unwrap()
  }
  fn getTomorrowEntries(dirname: String) -> Vec<sql::task::Task> {
    load_env(&dirname);
    get_tomorrow_tasks().unwrap()
  }
  fn updateDB(dirname: String) -> u32 {
      load_env(&dirname);
      update_db().unwrap()
  }
  fn getLastID(dirname: String) -> u32 {
    load_env(&dirname);
    let mut connection = establish_connection().unwrap();
    get_server_index_from_db(&mut connection)
  }
  fn getInboxEntries(dirname: String) -> Vec<sql::task::Task> {
    load_env(&dirname);
    get_inbox_tasks().unwrap()
  }
  fn parseFile(file_contents: String) -> bool {
    parse_file(&file_contents).unwrap()
  }
  fn get_canonical_id(input: String) -> String {
    sql::get_canonical_id(input)
  }
  fn get_sql_uuid(dirname: String, input: String) -> String {
    load_env(&dirname);
    println!("dirname: {:}", &dirname);
    let mut connection = establish_connection().expect("COnnection should be established");
    use sql::task::tasks::dsl::*;
    let db_entries = tasks
        .select(sql::canonical_id(uuid))
        .filter(title.eq(input))
        .load::<String>(&mut connection)
        .expect("Error loading Task");
        println!("db_entries: {:?}", db_entries);
        "OK".to_string()
  }
}

fn load_env(dirname: &str) {
    let mut dotenv_path = PathBuf::from(dirname);
    dotenv_path.push(".env");
    let path = dotenv_path.as_path();
    dotenv::from_path(path).ok();

    let mut database_url = PathBuf::from(dirname);
    database_url.push(env::var("DATABASE_URL").expect("DATABASE_URL must be set"));
    env::set_var("DATABASE_URL", database_url.to_str().unwrap());
}
fn parse_file(
    file_contents: &str,
) -> Result<bool, Box<dyn std::error::Error + Send + Sync + 'static>> {
    let _response: Response = serde_json::from_str(file_contents)?;
    Ok(true)
}
fn update_db() -> Result<u32, Box<dyn std::error::Error + Send + Sync + 'static>> {
    println!("Starting to Request history.");

    let mut connection = establish_connection()?;
    let mut index = get_server_index_from_db(&mut connection);
    let hist_id = get_hist_id();

    let client = reqwest::Client::new();

    loop {
        let url = format!(
            "{}{}{}{}",
            "https://cloud.culturedcode.com/version/1/history/",
            hist_id,
            "/items?start-index=",
            index
        );
        println!("Request: {:?}", index);
        let resp: Response = client
            .get(&url)
            .header(reqwest::header::USER_AGENT, "ThingsMac/30100506mas")
            .send()?
            .json()?;
        println!("Items: {:?}", resp.items.len());
        index += resp.items.len() as u32;

        let current_item_index = resp.current_item_index;
        insert_entries_into_db(&mut connection, resp)?;
        write_server_index_to_db(&mut connection, index)?;

        if index == current_item_index {
            println!(
                "index: {:?} current_item_index:{:?}",
                index, current_item_index
            );
            break;
        }
    }
    Ok(index)
}

fn insert_entries_into_db(
    connection: &mut SqliteConnection,
    resp: Response,
) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    for items_hash in resp.items.into_iter() {
        for (key, value) in items_hash.into_iter() {
            let canonical = sql::get_canonical_id(key);
            if !value.entity.starts_with("Task") {
                continue;
            }
            if value.item.is_none() {
                let item: Task =
                    serde_json::from_str(&format!("{}{}{}", r#"{"uuid":""#, canonical, r#""}"#))
                        .unwrap();
                delete(connection, item)?;
                continue;
            }
            let mut item = value.item.clone().unwrap();
            item.uuid = Some(canonical.to_owned());
            if value.operation_type == 2 {
                delete(connection, item)?;
            } else {
                insert_or_update(connection, item)?;
            }
        }
    }
    Ok(())
}

fn insert_or_update(
    connection: &mut SqliteConnection,
    item: Task,
) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    use sql::task::tasks::dsl::*;
    let db_entry = tasks
        .filter(
            uuid.eq(item
                .uuid
                .as_ref()
                .expect("uuid of a task to save can not be null.")),
        )
        .limit(1)
        .load::<sql::task::Task>(connection)
        .expect("Error loading Task");
    if db_entry.first().is_some() {
        let mut entry = db_entry[0].clone();
        entry.update_from(item);
        entry.save_changes::<sql::task::Task>(connection)?;
    } else {
        let entry = sql::task::Task::from(item);
        insert_into(tasks).values(&entry).execute(connection)?;
    }
    Ok(())
}

fn delete(
    connection: &mut SqliteConnection,
    item: Task,
) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    use sql::task::tasks::dsl::*;
    diesel::delete(
        tasks.filter(
            uuid.eq(item
                .uuid
                .expect("uuid of a task to remove can not be null.")),
        ),
    )
    .execute(connection)?;
    Ok(())
}

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");
fn run_embedded_migrations(
    connection: &mut SqliteConnection,
) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    connection.run_pending_migrations(MIGRATIONS)?;
    println!("Ran migrations");
    Ok(())
}

fn establish_connection(
) -> Result<SqliteConnection, Box<dyn std::error::Error + Send + Sync + 'static>> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut con = SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url));
    sql::register_sql_functions(&mut con);
    run_embedded_migrations(&mut con)?;
    Ok(con)
}

fn get_server_index_from_db(connection: &mut SqliteConnection) -> u32 {
    use sql::meta::meta::dsl::*;
    let results = meta
        .filter(key.eq("serverIndex"))
        .limit(1)
        .load::<Meta>(connection)
        .expect("Error loading meta");
    if results.first().is_some() {
        results.first().unwrap().value.parse().unwrap()
    } else {
        0
    }
}

fn write_server_index_to_db(
    connection: &mut SqliteConnection,
    index: u32,
) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    use sql::meta::meta::dsl::*;
    let results = meta
        .filter(key.eq("serverIndex"))
        .limit(1)
        .load::<Meta>(connection)
        .expect("Error loading meta");
    if results.first().is_some() {
        // update
        let mut entry: sql::meta::Meta = results[0].clone();
        entry.value = index.to_string();
        entry.save_changes::<sql::meta::Meta>(connection)?;
    } else {
        // insert
        let entry = sql::meta::Meta {
            key: "serverIndex".to_string(),
            value: index.to_string(),
        };
        insert_into(meta).values(&entry).execute(connection)?;
    }
    Ok(())
}

fn get_hist_id() -> String {
    env::var("HISTORY_ID").expect("HISTORY_ID must be set")
}

fn get_today_tasks(
) -> Result<Vec<sql::task::Task>, Box<dyn std::error::Error + Send + Sync + 'static>> {
    let mut connection = establish_connection()?;
    use sql::task::tasks::dsl::*;
    let today = get_today_0_0_timestamp();
    let db_entries = tasks
        .filter(trashed.eq(false))
        .filter(type_project.eq(0))
        .filter(start.eq(1))
        .filter(status.eq(0))
        .filter(today_index_reference_date.is_not_null())
        // .filter(today_index_reference_date.ne("asd").and(start_bucket.ne(1)))
        .order((today_index_reference_date.desc(), today_index.asc()))
        .load::<sql::task::Task>(&mut connection)
        .expect("Error loading Task");
    // select *  from TMTask where
    // -- select title, trashed, type, start, status, todayIndexReferenceDate, todayIndex, startBucket,  userModificationDate from TMTask where
    // trashed = false
    //  and "type" = 0
    //  and start = 1
    //  and status=0
    //  and todayIndexReferenceDate is not null
    //  and not (todayIndexReferenceDate = 132618496 and startBucket = 1)
    // order by todayIndexReferenceDate desc,  todayIndex asc
    // ;

    // select *  from TMTask where
    // trashed = false
    //  and "type" = 0
    //  and start = 1
    //  and status=0
    //  and (todayIndexReferenceDate = 132618496 and startBucket = 1)
    // order by todayIndexReferenceDate desc,  todayIndex asc
    Ok(db_entries)
}

fn get_tomorrow_tasks(
) -> Result<Vec<sql::task::Task>, Box<dyn std::error::Error + Send + Sync + 'static>> {
    let d = (SystemTime::now() + Duration::new(5 * 60 * 60, 0))
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let mut connection = establish_connection()?;
    use sql::task::tasks::dsl::*;
    let db_entries = tasks
        .filter(trashed.eq(false))
        .filter(type_project.eq(0))
        .filter(start.eq(2))
        .filter(status.eq(0))
        .filter(today_index_reference_date.lt(d as f64))
        .order((today_index_reference_date.desc(), today_index.asc()))
        .load::<sql::task::Task>(&mut connection)
        .expect("Error loading Task");
    Ok(db_entries)
}

fn get_inbox_tasks(
) -> Result<Vec<sql::task::Task>, Box<dyn std::error::Error + Send + Sync + 'static>> {
    let mut connection = establish_connection()?;
    use sql::task::tasks::dsl::*;
    let db_entries = tasks
        .filter(trashed.eq(false))
        .filter(type_project.eq(0))
        .filter(start.eq(0))
        .filter(status.eq(0))
        .order(index.desc())
        .load::<sql::task::Task>(&mut connection)
        .expect("Error loading Task");
    Ok(db_entries)
}

fn get_today_0_0_timestamp() -> i64 {
    use chrono::prelude::*;

    let utc: DateTime<Utc> = Utc::now();
    utc.with_hour(0)
        .unwrap()
        .with_minute(0)
        .unwrap()
        .with_second(0)
        .unwrap()
        .with_nanosecond(0)
        .unwrap()
        .timestamp()
}
