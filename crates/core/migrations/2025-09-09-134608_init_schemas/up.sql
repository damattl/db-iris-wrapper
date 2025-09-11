-- Your SQL goes here
-- Stations
CREATE TABLE stations (
    id      INTEGER PRIMARY KEY,
    lat     DOUBLE PRECISION,
    lon     DOUBLE PRECISION,
    name    TEXT NOT NULL,
    ds100   TEXT NOT NULL,
    CONSTRAINT stations_lat_range CHECK (lat IS NULL OR lat BETWEEN -90 AND 90),
    CONSTRAINT stations_lon_range CHECK (lon IS NULL OR lon BETWEEN -180 AND 180),
    CONSTRAINT stations_ds100_len CHECK (char_length(ds100) BETWEEN 1 AND 16)
);

CREATE INDEX stations_ds100_idx ON stations (ds100);

-- Trains
CREATE TABLE trains (
    id        TEXT PRIMARY KEY,          -- e.g., "<number>-<date>"
    operator  TEXT,
    category  TEXT NOT NULL,
    number    TEXT NOT NULL,
    line      TEXT,
    date      DATE NOT NULL
);

CREATE INDEX trains_date_idx ON trains (date);
CREATE INDEX trains_cat_num_idx ON trains (category, number);

-- Messages
CREATE TABLE messages (
    id          TEXT PRIMARY KEY,
    train_id    TEXT NOT NULL REFERENCES trains(id) ON DELETE CASCADE,
    valid_from  TIMESTAMP,
    valid_to    TIMESTAMP,
    priority    SMALLINT,                -- u8 -> SMALLINT; enforce 0..255:
    category    TEXT,
    code        INTEGER,
    "timestamp" TIMESTAMP NOT NULL,
    m_type      TEXT,
    CONSTRAINT messages_priority_range CHECK (priority IS NULL OR priority BETWEEN 0 AND 255)
);

CREATE INDEX messages_train_id_idx ON messages (train_id);
CREATE INDEX messages_ts_idx ON messages ("timestamp");
