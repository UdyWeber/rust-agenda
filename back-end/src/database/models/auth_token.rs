use std::error::Error;

use chrono::NaiveDateTime;
use diesel::{
    Associations, ExpressionMethods, Identifiable, Insertable, JoinOnDsl, QueryDsl, Queryable,
    Selectable, SelectableHelper,
};
use diesel_async::RunQueryDsl;
use serde::Serialize;
use uuid::Uuid;

use crate::{
    database::{
        db::DbConnection,
        models::user::SelectableUser,
        schema::{auth_token, users},
    },
    utils::security::generate_random_token,
};

#[derive(Serialize, Identifiable, Queryable, Debug, Selectable, PartialEq, Associations)]
#[diesel(belongs_to(SelectableUser, foreign_key = user_id))]
#[diesel(table_name = auth_token)]
pub struct SelectableAuthToken {
    id: Uuid,
    user_id: Uuid,
    token: String,
    date_created: Option<NaiveDateTime>,
}

#[derive(Insertable, Serialize, Clone)]
#[diesel(table_name = auth_token)]
pub struct InsertableAuthToken {
    user_id: Uuid,
    token: String,
}

impl InsertableAuthToken {
    pub fn new(user_id: Uuid) -> Self {
        let token = generate_random_token();

        Self {
            user_id: user_id,
            token: token,
        }
    }
}

pub struct AuthTokenQueries<'a> {
    pub connection: DbConnection<'a>,
}

impl AuthTokenQueries<'_> {
    pub async fn get_auth_user_by_token(
        &mut self,
        token: String,
    ) -> Result<SelectableUser, impl Error> {
        users::table
            .inner_join(auth_token::table.on(auth_token::user_id.eq(users::id)))
            .select(SelectableUser::as_select())
            .filter(auth_token::token.eq(token))
            .first::<SelectableUser>(&mut self.connection)
            .await
    }
    pub async fn insert_auth_token(&mut self, token_data: InsertableAuthToken) {
        let _ = diesel::insert_into(auth_token::table)
            .values(token_data)
            .execute(&mut self.connection)
            .await;
    }
}
