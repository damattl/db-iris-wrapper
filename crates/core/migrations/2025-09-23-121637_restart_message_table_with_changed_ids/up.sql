-- Your SQL goes here
DELETE FROM messages_to_stations;
DELETE FROM messages;
ALTER TABLE messages ADD COLUMN iris_id TEXT NOT NULL;
