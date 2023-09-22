table! {
    meta (key) {
        key -> Varchar,
        value -> Varchar,
    }
}

#[derive(Clone, Insertable, Queryable, Identifiable, AsChangeset)]
#[diesel(primary_key(key))]
#[diesel(table_name = meta)]
pub struct Meta {
    pub key: String,
    pub value: String,
}
