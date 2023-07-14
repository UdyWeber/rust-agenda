use std::error::Error;

use chrono::NaiveDateTime;
use diesel::{ExpressionMethods, Identifiable, Insertable, QueryDsl, Queryable, Selectable};
use diesel_async::RunQueryDsl;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    database::{
        db::DbConnection,
        schema::{auth_token, users},
    },
    utils::security::generate_password_salt,
};

use super::auth_token::InsertableAuthToken;

// REMEMBER: To query the structure you must have the whole structure defined here no excuses
#[derive(Serialize, Queryable, Debug, Identifiable, Clone, PartialEq, Selectable)]
#[diesel(table_name = users)]
pub struct SelectableUser {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String,
    #[serde(skip_serializing)]
    pub password_salt: String,
    pub date_created: Option<NaiveDateTime>,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = users)]
pub struct InsertableUser {
    name: String,
    pub email: String,
    #[serde(skip_serializing)]
    password: String,
    #[serde(skip_serializing, skip_deserializing)]
    password_salt: String,
}

impl InsertableUser {
    pub fn encrypt_password(&mut self) {
        let (salt, password) = generate_password_salt(&self.password);
        self.password = password;
        self.password_salt = salt;
    }
}

pub struct UserQueries<'a> {
    pub connection: DbConnection<'a>,
}

impl UserQueries<'_> {
    pub async fn get_user_by_email(&mut self, email: String) -> Result<SelectableUser, impl Error> {
        users::table
            .filter(users::email.eq(email))
            .get_result(&mut self.connection)
            .await
    }
    pub async fn inser_user(
        &mut self,
        mut user_data: InsertableUser,
    ) -> Result<SelectableUser, impl Error> {
        user_data.encrypt_password();

        diesel::insert_into(users::table)
            .values(user_data)
            .get_result::<SelectableUser>(&mut self.connection)
            .await
    }
    pub async fn insert_auth_token(&mut self, token_data: InsertableAuthToken) {
        let _ = diesel::insert_into(auth_token::table)
            .values(token_data)
            .execute(&mut self.connection)
            .await;
    }
}

#[derive(Serialize, Deserialize)]
pub struct LoginData {
    pub email: String,
    pub password: String,
}
