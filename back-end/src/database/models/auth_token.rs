use chrono::NaiveDateTime;
use diesel::{Associations, Identifiable, Insertable, Queryable, Selectable};
use serde::Serialize;
use uuid::Uuid;

use crate::database::schema::auth_token::{self};

use super::user::SelectableUser;

#[derive(Serialize, Identifiable, Queryable, Debug, Selectable, PartialEq, Associations)]
#[diesel(belongs_to(SelectableUser, foreign_key = user_id))]
#[diesel(table_name = auth_token)]
pub struct SelectableAuthToken {
    id: Uuid,
    user_id: Uuid,
    token: String,
    date_created: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = auth_token)]
pub struct InsertableAuthToken {
    user_id: Uuid,
    token: String,
}
