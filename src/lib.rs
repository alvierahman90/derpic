pub mod db;
pub mod models;
pub mod schema;

use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use rand::prelude::*;
use std::env;

pub fn random_bytes(len: usize) -> Vec<u8> {
    let mut bytes = vec![0; len];
    rand::rngs::OsRng.fill_bytes(&mut bytes);

    bytes
}

pub fn random_string(len: usize) -> String {
    URL_SAFE.encode(random_bytes(len))
}

pub fn static_files_directory() -> String {
    env::var("DERPIC_STATIC_FILES").unwrap_or("/opt/derpic/src-web".into())
}
