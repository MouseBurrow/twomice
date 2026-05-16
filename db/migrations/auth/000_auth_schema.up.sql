CREATE EXTENSION IF NOT EXISTS pgcrypto;

CREATE TABLE accounts
(
    id            UUID PRIMARY KEY   NOT NULL DEFAULT gen_random_uuid(),
    username      VARCHAR(50) UNIQUE NOT NULL,
    password_hash TEXT               NOT NULL,
    is_admin      BOOLEAN            NOT NULL DEFAULT FALSE,
    created_at    TIMESTAMPTZ                 DEFAULT NOW(),
    updated_at    TIMESTAMPTZ                 DEFAULT NOW()
);

CREATE TABLE sessions
(
    id            UUID PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
    account_id    UUID             NOT NULL REFERENCES accounts (id) ON DELETE CASCADE,
    session_token TEXT UNIQUE      NOT NULL,
    last_used_at  TIMESTAMP                 DEFAULT NOW(),
    expires_at    TIMESTAMP                 DEFAULT (NOW() + INTERVAL '30 days')
);
