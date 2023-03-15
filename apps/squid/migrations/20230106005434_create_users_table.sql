-- Add migration script here
CREATE TABLE users(
    id uuid NOT NULL,
    PRIMARY KEY (id),
    username TEXT NOT NULL UNIQUE,
    password TEXT NOT NULL
)
