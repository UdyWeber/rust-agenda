-- Your SQL goes here


CREATE TABLE IF NOT EXISTS users (
    id uuid DEFAULT gen_random_uuid() PRIMARY KEY,
    name varchar(255) NOT NULL,
    email varchar(255) NOT NULL,
    password text NOT NULL,
    password_salt varchar(255) NOT NULL,
    date_created timestamp default now()
);


CREATE TABLE IF NOT EXISTS auth_token (
    id uuid DEFAULT gen_random_uuid() PRIMARY KEY,
    user_id uuid NOT NULL,
    token varchar(255) NOT NULL,
    date_created timestamp default now(),
    constraint fk_user_token
        foreign key (user_id)
            references users(id)
);

CREATE TABLE IF NOT EXISTS cards (
    id uuid DEFAULT gen_random_uuid() PRIMARY KEY,
    user_id uuid NOT NULL,
    content text NOT NULL,
    date_created timestamp default now(),
    constraint fk_user_card
        foreign key (user_id)
            references users(id)
)
