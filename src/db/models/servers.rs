use crate::db::schema;
use diesel::prelude::*;

#[allow(dead_code)]
#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = schema::servers)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Server {
    pub id: i64,
    pub verified_role_id: Option<i64>,
}

#[allow(dead_code)]
#[derive(Debug, Insertable)]
#[diesel(table_name = schema::servers)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewServer {
    pub id: i64,
}
