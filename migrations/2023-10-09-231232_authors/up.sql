-- Your SQL goes here
CREATE TABLE authors (
                           id SERIAL PRIMARY KEY,
                           author_name text unique not NULL
);