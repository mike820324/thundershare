-- Add migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE customer (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    username TEXT NOT NULL,
    password TEXT NOT NULL,
    UNIQUE(id),
    UNIQUE(username),
    UNIQUE(password)
);
