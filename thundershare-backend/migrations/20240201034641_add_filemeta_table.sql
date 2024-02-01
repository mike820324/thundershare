-- Add migration script here
CREATE TABLE filemeta (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    customer_id UUID,
    url TEXT,
    UNIQUE(url),
    FOREIGN KEY(customer_id) REFERENCES customer(id)
);
