use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};
use serde::Serialize;
use uuid::Uuid;

use crate::database::schema::auth_token::{self};

#[derive(Serialize, Queryable, Debug)]
pub struct SelectableAuthToken {
    id: Uuid,
    user_id: Uuid,
    token: String,
    date_created: NaiveDateTime
}

#[derive(Insertable)]
#[diesel(table_name = auth_token)]
pub struct InsertableAuthToken {
    user_id: Uuid,
    token: String
}