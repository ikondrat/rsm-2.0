use diesel::prelude::*;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::schema::users;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub username: &'a str,
    pub email: &'a str,
}