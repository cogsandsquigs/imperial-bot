// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "user_state"))]
    pub struct UserState;
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
