// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "image_format"))]
    pub struct ImageFormat;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ImageFormat;

    images (id) {
        id -> Int4,
        token_id -> Int4,
        image -> Bytea,
        format -> ImageFormat,
        slug -> Bytea,
    }
}

diesel::table! {
    tokens (id) {
        id -> Int4,
        token -> Varchar,
        name -> Varchar,
        revoked -> Bool,
    }
}

diesel::joinable!(images -> tokens (token_id));

diesel::allow_tables_to_appear_in_same_query!(
    images,
    tokens,
);
