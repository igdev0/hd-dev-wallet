CREATE TABLE accounts (
    id UUID PRIMARY KEY,
    wallet_id UUID REFERENCES wallets(id),
    account_index INT NOT NULL,
    xpub TEXT NOT NULL,
    xprv TEXT DEFAULT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);