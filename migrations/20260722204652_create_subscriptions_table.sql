-- Add migration script here
CREATE TABLE IF NOT EXISTS subscriptions (
    id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    user_name TEXT NOT NULL UNIQUE,
    email TEXT NOT NULL,

    subscribed_at TIMESTAMPTZ NOT NULL
);
