// @generated automatically by Diesel CLI.

diesel::table! {
    images (id) {
        id -> Int4,
        token_id -> Int4,
        image -> Bytea,
        slug -> Bytea,
    }
}

diesel::table! {
    tokens (id) {
        id -> Int4,
        token -> Bytea,
        name -> Varchar,
        revoked -> Bool,
    }
}

diesel::joinable!(images -> tokens (token_id));

diesel::allow_tables_to_appear_in_same_query!(
    images,
    tokens,
);
