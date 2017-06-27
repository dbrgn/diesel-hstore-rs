CREATE EXTENSION hstore;
CREATE TABLE animals (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    attributes HSTORE NOT NULL
);
