// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "user_state"))]
    pub struct UserState;
}

diesel::table! {
    servers (id) {
        id -> Int8,
        verified_role_id -> Nullable<Int8>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::UserState;

    users (id) {
        id -> Int8,
        imperial_email -> Nullable<Varchar>,
        state -> UserState,
        otps -> Array<Nullable<Int4>>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    servers,
    users,
);
