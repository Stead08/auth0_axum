-- Add migration script here
CREATE TABLE IF NOT EXISTS users (
                       id SERIAL NOT NULL PRIMARY KEY,
                       first_name TEXT NOT NULL,
                       last_name TEXT NOT NULL,
                       email TEXT NOT NULL,
                       created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);