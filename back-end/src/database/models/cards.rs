use chrono::NaiveDateTime;
use diesel::{Associations, Identifiable, Insertable, Queryable, Selectable};
use serde::Serialize;
use uuid::Uuid;

use crate::database::schema::cards::{self};

use super::user::SelectableUser;

#[derive(Serialize, Queryable, Identifiable, Debug, Selectable, PartialEq, Associations)]
#[diesel(belongs_to(SelectableUser, foreign_key = user_id))]
#[diesel(table_name = cards)]
pub struct SelectableCard {
    id: Uuid,
    user_id: Uuid,
    content: String,
    date_created: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = cards)]
pub struct InsertableCards {
    user_id: Uuid,
    content: String,
}
