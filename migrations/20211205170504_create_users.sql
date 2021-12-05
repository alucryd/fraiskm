CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS users (
    id UUID NOT NULL PRIMARY KEY DEFAULT uuid_generate_v4(),
    username VARCHAR(32) UNIQUE NOT NULL,
    password_hash VARCHAR(128) NOT NULL
);