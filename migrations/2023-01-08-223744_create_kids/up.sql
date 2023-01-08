-- Your SQL goes here
CREATE TABLE kids (
    id SERIAL PRIMARY KEY,
    name text NOT NULL,
    initial_balance float NOT NULL,
    user_id SERIAL REFERENCES users(id)
)
