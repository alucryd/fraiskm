CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS drivers (
    id UUID NOT NULL PRIMARY KEY DEFAULT uuid_generate_v4(),
    name BYTEA NOT NULL,
    limit_distance BOOLEAN NOT NULL DEFAULT TRUE,
    user_id UUID NOT NULL,
    default_vehicle_id UUID,
    default_from_id UUID,
    default_to_id UUID,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (default_vehicle_id) REFERENCES vehicles(id) ON DELETE SET NULL,
    FOREIGN KEY (default_from_id) REFERENCES addresses(id) ON DELETE SET NULL,
    FOREIGN KEY (default_to_id) REFERENCES addresses(id) ON DELETE SET NULL
);