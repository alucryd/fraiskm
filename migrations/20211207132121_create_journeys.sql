CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS journeys (
    id UUID NOT NULL PRIMARY KEY,
    from_id UUID NOT NULL,
    to_id UUID NOT NULL,
    driver_id UUID NOT NULL,
    vehicle_id UUID NOT NULL,
    date DATE NOT NULL,
    meters INTEGER NOT NULL,
    FOREIGN KEY (from_id) REFERENCES addresses(id) ON DELETE CASCADE,
    FOREIGN KEY (to_id) REFERENCES addresses(id) ON DELETE CASCADE,
    FOREIGN KEY (driver_id) REFERENCES drivers(id) ON DELETE CASCADE,
    FOREIGN KEY (vehicle_id) REFERENCES vehicles(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS journeys_date ON journeys (date);