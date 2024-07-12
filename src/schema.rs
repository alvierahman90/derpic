// @generated automatically by Diesel CLI.

diesel::table! {
    tokens (id) {
        id -> Int4,
        token -> Varchar,
        name -> Varchar,
        revoked -> Bool,
    }
}
