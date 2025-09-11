-- Your SQL goes here
-- Stops (Movement fields inlined)
CREATE TABLE stops (
    id                      TEXT PRIMARY KEY,
    train_id                TEXT    NOT NULL REFERENCES trains(id)   ON DELETE CASCADE,
    station_id              INTEGER NOT NULL REFERENCES stations(id) ON DELETE RESTRICT,

    -- arrival movement
    arrival_platform        TEXT,
    arrival_planned         TIMESTAMP,
    arrival_planned_path    TEXT,      -- Option<Vec<String>>
    arrival_changed_path    TEXT,

    -- departure movement
    departure_platform      TEXT,
    departure_planned       TIMESTAMP,
    departure_planned_path  TEXT,
    departure_changed_path  TEXT
);

-- Useful indexes for common lookups
CREATE INDEX stops_train_id_idx          ON stops (train_id);
CREATE INDEX stops_station_id_idx        ON stops (station_id);
CREATE INDEX stops_train_station_idx     ON stops (train_id, station_id);
CREATE INDEX stops_arrival_planned_idx   ON stops (arrival_planned);
CREATE INDEX stops_departure_planned_idx ON stops (departure_planned);
