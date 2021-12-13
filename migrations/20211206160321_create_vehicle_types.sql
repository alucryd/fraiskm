CREATE TABLE IF NOT EXISTS vehicle_types (
    id SMALLSERIAL PRIMARY KEY,
    label VARCHAR(64) NOT NULL
);

INSERT INTO vehicle_types (id, label) VALUES
    (1, 'Voiture'),
    (2, 'Deux roues de cylindrée inférieure à 50 cm3'),
    (3, 'Moto');