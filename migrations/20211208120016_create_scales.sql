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
    vehicle_type_id SMALLSERIAL NOT NULL, 
    FOREIGN KEY (vehicle_type_id) REFERENCES vehicle_types(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS scales_year_horsepower_vehicle_type ON scales (year, horsepower_min, horsepower_max, vehicle_type_id);

INSERT INTO scales (year, horsepower_min, horsepower_max, first_threshold, second_threshold, first_slice_multiplier, second_slice_multiplier, third_slice_multiplier, second_slice_fixed_amount, vehicle_type_id) VALUES
    (2021, NULL, 3, 5000, 20000, 0.456, 0.273, 0.318, 915, 1),
    (2021, 4, 4, 5000, 20000, 0.523, 0.294, 0.352, 1147, 1),
    (2021, 5, 5, 5000, 20000, 0.548, 0.308, 0.368, 1200, 1),
    (2021, 6, 6, 5000, 20000, 0.574, 0.323, 0.386, 1256, 1),
    (2021, 7, NULL, 5000, 20000, 0.601, 0.340, 0.405, 1301, 1),
    (2021, NULL, NULL, 3000, 6000, 0.272, 0.064, 0.147, 416, 2),
    (2021, NULL, 2, 3000, 6000, 0.341, 0.085, 0.213, 768, 3),
    (2021, 3, 5, 3000, 6000, 0.404, 0.071, 0.237, 999, 3),
    (2021, 6, NULL, 3000, 6000, 0.523, 0.068, 0.295, 1365, 3);