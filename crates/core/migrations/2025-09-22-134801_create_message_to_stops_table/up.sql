-- Your SQL goes here
CREATE TABLE messages_to_stations (
    station_id              INTEGER NOT NULL REFERENCES stations(id)   ON DELETE RESTRICT,
    message_id              TEXT NOT NULL REFERENCES messages(id) ON DELETE RESTRICT,
    PRIMARY KEY (message_id, station_id)
);
