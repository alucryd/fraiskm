CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS scales (
    id UUID NOT NULL PRIMARY KEY DEFAULT uuid_generate_v4(),
    year SMALLINT NOT NULL,
    horsepower_min SMALLINT,
    horsepower_max SMALLINT,
    first_threshold SMALLINT NOT NULL,
    second_threshold SMALLINT NOT NULL,
    first_slice_multiplier NUMERIC(4, 3) NOT NULL,
    second_slice_multiplier NUMERIC(4, 3) NOT NULL,
    third_slice_multiplier NUMERIC(4, 3) NOT NULL,
    second_slice_fixed_amount SMALLINT NOT NULL,
    vehicle_type SMALLINT NOT NULL
);

CREATE INDEX IF NOT EXISTS scales_year_horsepower_vehicle_type ON scales (year, horsepower_min, horsepower_max, vehicle_type);