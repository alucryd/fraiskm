CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS cars (
    id UUID NOT NULL PRIMARY KEY DEFAULT uuid_generate_v4(),
    model BYTEA NOT NULL,
    horsepower SMALLINT NOT NULL,
    user_id UUID NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);