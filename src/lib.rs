pub mod db;
pub mod models;
pub mod schema;

use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use rand::prelude::*;
pub fn random_bytes(len: usize) -> Vec<u8> {
    let mut bytes = vec![0; len];
    rand::rngs::OsRng.fill_bytes(&mut bytes);

    bytes
}

pub fn random_string(len: usize) -> String {
    URL_SAFE.encode(random_bytes(len))
}

pub mod env {
    use std::env;

    pub fn static_files_directory() -> String {
        env::var("DERPIC_STATIC_FILES").unwrap_or("/opt/derpic/src-web".into())
    }

    pub fn public_base_url() -> String {
        env::var("DERPIC_PUBLIC_BASE_URL")
            .expect("DERPIC_PUBLIC_BASE_URL environment variable not defined")
    }

    pub fn admin_token() -> String {
        env::var("DERPIC_ADMIN_TOKEN").expect("DERPIC_ADMIN_TOKEN environment variable not defined")
    }
}
