-- This file should undo anything in `up.sql`
ALTER TABLE kids ADD COLUMN initial_balance float NOT NULL;
