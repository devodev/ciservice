// @generated automatically by Diesel CLI.

diesel::table! {
    job (id) {
        id -> Int4,
        name -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
