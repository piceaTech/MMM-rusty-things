pub mod meta;
pub mod task;

use diesel::sql_types::Text;
use diesel::sqlite::SqliteConnection;
use sha1::{Digest, Sha1};

sql_function!(fn canonical_id(input: Text) -> Text);

pub fn register_sql_functions(connection: &mut SqliteConnection) {
    canonical_id::register_impl(connection, get_canonical_id).unwrap();
}

pub fn get_canonical_id(input: String) -> String {
    if input.contains("-") {
        if input.len() == 45 {
            let sha = Sha1::digest(&input.as_bytes()[0..36]);
            let mut arr = vec![0u8; 25];
            arr.splice(..16, sha.as_slice()[0..16].iter().cloned());
            arr.splice(16.., input.as_bytes()[36..].iter().cloned());
            let sha2 = Sha1::digest(&arr);
            let mut result = vec![0; 16];
            result.clone_from_slice(&sha2.as_slice()[0..16]);
            bs58::encode(result).into_string()
        } else {
            let sha = Sha1::digest(input.as_bytes());
            let mut arr = vec![0; 16];
            arr.clone_from_slice(&sha.as_slice()[0..16]);
            bs58::encode(arr).into_string()
        }
    } else {
        input
    }
}
