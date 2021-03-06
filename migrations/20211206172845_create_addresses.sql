CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS addresses (
    id UUID NOT NULL PRIMARY KEY DEFAULT uuid_generate_v4(),
    title BYTEA NOT NULL,
    label BYTEA NOT NULL,
    address_type SMALLINT NOT NULL, 
    user_id UUID NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);