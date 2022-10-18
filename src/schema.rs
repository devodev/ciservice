// @generated automatically by Diesel CLI.

diesel::table! {
    job (id) {
        id -> Int4,
        name -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    job_parameter (id) {
        id -> Int4,
        job_id -> Int4,
        name -> Text,
        value -> Text,
        #[sql_name = "type"]
        type_ -> Text,
        created_at -> Timestamp,
    }
}

diesel::joinable!(job_parameter -> job (job_id));

diesel::allow_tables_to_appear_in_same_query!(
    job,
    job_parameter,
);
