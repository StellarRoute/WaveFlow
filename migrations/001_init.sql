# Initial Postgres schema for WaveFlow programs, contributors, payouts, and webhook audit.
-- WaveFlow migration 001: core tables

CREATE TABLE IF NOT EXISTS programs (
    id UUID PRIMARY KEY,
    on_chain_program_id BIGINT NOT NULL UNIQUE,
    github_repo TEXT NOT NULL UNIQUE,
    maintainer_address TEXT NOT NULL,
    reward_per_point BIGINT NOT NULL CHECK (reward_per_point > 0),
    escrow_balance BIGINT NOT NULL DEFAULT 0,
    milestone_cap BIGINT,
    milestone_spent BIGINT NOT NULL DEFAULT 0,
    status TEXT NOT NULL DEFAULT 'active' CHECK (status IN ('active', 'paused')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS contributors (
    program_id UUID NOT NULL REFERENCES programs(id) ON DELETE CASCADE,
    github_username TEXT NOT NULL,
    stellar_address TEXT NOT NULL,
    registered_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (program_id, github_username)
);

CREATE TABLE IF NOT EXISTS payouts (
    id UUID PRIMARY KEY,
    program_id UUID NOT NULL REFERENCES programs(id) ON DELETE CASCADE,
    pr_number BIGINT NOT NULL,
    github_username TEXT NOT NULL,
    stellar_address TEXT NOT NULL,
    points INTEGER NOT NULL CHECK (points > 0),
    amount BIGINT NOT NULL CHECK (amount > 0),
    tx_hash TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (program_id, pr_number)
);

CREATE TABLE IF NOT EXISTS webhook_events (
    id UUID PRIMARY KEY,
    delivery_id TEXT,
    event_type TEXT NOT NULL,
    github_repo TEXT NOT NULL,
    pr_number BIGINT,
    payload JSONB NOT NULL,
    status TEXT NOT NULL,
    error_message TEXT,
    received_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_payouts_program_id ON payouts(program_id);
CREATE INDEX IF NOT EXISTS idx_webhook_events_repo ON webhook_events(github_repo);
