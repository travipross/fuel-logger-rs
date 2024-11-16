-- Add up migration script here

CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL,
    username TEXT NOT NULL UNIQUE,
    email TEXT NOT NULL UNIQUE
);

CREATE TABLE vehicles (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    owner_id UUID REFERENCES users(id),
    make TEXT NOT NULL,
    model TEXT NOT NULL,
    year INTEGER NOT NULL,
    odometer_unit TEXT NOT NULL
);

CREATE TABLE log_records (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    vehicle_id UUID REFERENCES vehicles(id),
    log_date TIMESTAMP WITH TIME ZONE NOT NULL,
    odometer INTEGER NOT NULL,
    log_type TEXT NOT NULL,
    fuel_amount REAL,
    notes TEXT,
    rotation_type TEXT,
    tire_type TEXT,
    new_tires TEXT,
    brake_location TEXT,
    brake_part TEXT,
    fluid_type TEXT
);
