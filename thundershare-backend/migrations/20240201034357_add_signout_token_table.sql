-- Add migration script here
CREATE TABLE signouttoken (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    token TEXT NOT NULL,
    expiretime TIMETZ,
    UNIQUE(token)
);
