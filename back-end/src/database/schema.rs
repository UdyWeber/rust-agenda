// @generated automatically by Diesel CLI.

diesel::table! {
    auth_token (id) {
        id -> Uuid,
        user_id -> Uuid,
        token -> Varchar,
        date_created -> Nullable<Timestamp>,
    }
}

diesel::table! {
    cards (id) {
        id -> Uuid,
        user_id -> Uuid,
        content -> Text,
        date_created -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        name -> Varchar,
        email -> Varchar,
        password -> Text,
        password_salt -> Varchar,
        date_created -> Nullable<Timestamp>,
    }
}

diesel::joinable!(auth_token -> users (user_id));
diesel::joinable!(cards -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(auth_token, cards, users,);
