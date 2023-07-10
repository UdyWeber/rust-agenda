use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};
use serde::Serialize;
use uuid::Uuid;

use crate::database::schema::users::{self};

#[derive(Serialize, Queryable, Debug)]
pub struct SelectableUser {
    id: Uuid,
    name: String,
    email: String,
    date_created: NaiveDateTime
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct InsertableUser {
    name: String,
    email: String,
    password: String,
    password_salt: String,
}