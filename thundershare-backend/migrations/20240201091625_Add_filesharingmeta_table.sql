-- Add migration script here
CREATE TABLE filesharingmeta (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    file_id UUID,
    link TEXT,
    expireat timestamptz,
    password TEXT,
    FOREIGN KEY(file_id) REFERENCES filemeta(id)
);

