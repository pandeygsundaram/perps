-- Add migration script here
CREATE TABLE fills (
    id SERIAL PRIMARY KEY,

    taker INTEGER NOT NULL
        REFERENCES users(id),

    maker INTEGER NOT NULL
        REFERENCES users(id),

    market_id INTEGER NOT NULL
        REFERENCES markets(id),

    price BIGINT NOT NULL,

    qty BIGINT NOT NULL,

    maker_original_id INTEGER NOT NULL
        REFERENCES orders(id),

    taker_original_id INTEGER NOT NULL
        REFERENCES orders(id),

    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);