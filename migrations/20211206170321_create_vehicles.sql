CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS vehicles (
    id UUID NOT NULL PRIMARY KEY DEFAULT uuid_generate_v4(),
    model BYTEA NOT NULL,
    horsepower SMALLINT NOT NULL,
    electric BOOLEAN NOT NULL DEFAULT FALSE,
    vehicle_type_id SMALLSERIAL NOT NULL, 
    user_id UUID NOT NULL,
    FOREIGN KEY (vehicle_type_id) REFERENCES vehicle_types(id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);