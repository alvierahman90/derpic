pub mod db;
pub mod models;
pub mod schema;

use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use rand::prelude::*;


pub fn random_string(len: usize) -> String {
    let mut bytes = vec![0; len];
    rand::rngs::OsRng.fill_bytes(&mut bytes);
    URL_SAFE.encode(bytes)
}
