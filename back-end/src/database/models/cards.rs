use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};
use serde::Serialize;
use uuid::Uuid;

use crate::database::schema::cards::{self};

#[derive(Serialize, Queryable, Debug)]
pub struct SelectableCard {
    id: Uuid,
    user_id: Uuid,
    content: String,
    date_created: NaiveDateTime
}

#[derive(Insertable)]
#[diesel(table_name = cards)]
pub struct InsertableCards {
    user_id: Uuid,
    content: String,
}