CREATE TABLE transactions (
    id SERIAL PRIMARY KEY,
    ts TIMESTAMP NOT NULL,
    change float NOT NULL,
    kid_id SERIAL REFERENCES kids(id)
)