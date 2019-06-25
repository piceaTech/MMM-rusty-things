table! {
    meta (key) {
        key -> Varchar,
        value -> Varchar,
    }
}


#[derive(Clone)]
#[derive(Insertable)]
#[derive(Queryable, Identifiable, AsChangeset)]
#[primary_key(key)]
#[table_name="meta"]
pub struct Meta {
    pub key: String,
    pub value: String,
}