CREATE TABLE wallets (
    id UUID PRIMARY KEY,
    seed TEXT NOT NULL,
    password TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);