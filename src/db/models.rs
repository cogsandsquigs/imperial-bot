use super::schema;
use diesel::prelude::*;
use diesel_derive_enum::DbEnum;

#[allow(dead_code)]
#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i64,
    pub imperial_email: Option<String>,
    pub state: UserState,
    pub otps: Vec<Option<i32>>,
}

#[derive(Insertable)]
#[diesel(table_name = schema::users)]
pub struct NewUser {
    pub id: i64,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, DbEnum, PartialEq, Eq)]
#[ExistingTypePath = "super::schema::sql_types::UserState"]
pub enum UserState {
    Unverified = 0,
    QueryingEmail = 1,
    QueryingOTP = 2,
    Verified = 3,
}
