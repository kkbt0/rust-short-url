-- Your SQL goes here
CREATE TABLE url_table (
    id INTEGER NOT NULL PRIMARY KEY,
    redirect_url Text NOT NULL UNIQUE
)
