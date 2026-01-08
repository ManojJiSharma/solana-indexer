CREATE TABLE transactions (
    signature TEXT PRIMARY KEY,
    slot BIGINT NOT NULL,
    logs TEXT[],
    created_at TIMESTAMP DEFAULT now()
);

CREATE INDEX idx_slot ON transactions(slot);
