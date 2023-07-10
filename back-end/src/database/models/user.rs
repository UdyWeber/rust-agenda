use chrono::NaiveDateTime;
use diesel::{Identifiable, Insertable, Queryable, Selectable};
use serde::Serialize;
use uuid::Uuid;

use crate::database::schema::users::{self};

// REMEMBER: To query the structure you must have the whole structure defined here no excuses
#[derive(Serialize, Queryable, Debug, Identifiable, Clone, PartialEq, Selectable)]
#[diesel(table_name = users)]
pub struct SelectableUser {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    #[serde(skip_serializing)]
    password: String,
    #[serde(skip_serializing)]
    password_salt: String,
    pub date_created: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct InsertableUser {
    name: String,
    email: String,
    password: String,
    password_salt: String,
}
