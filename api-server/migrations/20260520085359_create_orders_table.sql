-- Add migration script here
CREATE TYPE order_type AS ENUM (
    'LIMIT',
    'MARKET'
);

CREATE TYPE order_side AS ENUM (
    'LONG',
    'SHORT'
);

CREATE TABLE orders (
    id SERIAL PRIMARY KEY,

    user_id INTEGER NOT NULL
        REFERENCES users(id),

    market_id INTEGER NOT NULL
        REFERENCES markets(id),

    type order_type NOT NULL,

    side order_side NOT NULL,

    price BIGINT NOT NULL,

    qty BIGINT NOT NULL,

    filled_qty BIGINT NOT NULL DEFAULT 0,

    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);