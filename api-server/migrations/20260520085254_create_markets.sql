-- Add migration script here
CREATE TABLE markets (
    id SERIAL PRIMARY KEY,
    market_slug TEXT UNIQUE NOT NULL,
    image_url TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);