CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS distances (
    from_id UUID NOT NULL,
    to_id UUID NOT NULL,
    meters INTEGER NOT NULL,
    FOREIGN KEY (from_id) REFERENCES addresses(id) ON DELETE CASCADE,
    FOREIGN KEY (to_id) REFERENCES addresses(id) ON DELETE CASCADE,
    PRIMARY KEY(from_id, to_id)
);