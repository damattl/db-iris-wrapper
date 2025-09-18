-- This file should undo anything in `up.sql`
ALTER TABLE stops DROP COLUMN arrival_current;
ALTER TABLE stops DROP COLUMN departure_current;
