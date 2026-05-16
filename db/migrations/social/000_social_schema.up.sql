CREATE EXTENSION IF NOT EXISTS pgcrypto;

CREATE TABLE friend_requests
(
    id          UUID PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
    sender_id   UUID             NOT NULL,
    receiver_id UUID             NOT NULL,
    status      TEXT             NOT NULL DEFAULT 'pending',
    created_at  TIMESTAMPTZ               DEFAULT NOW(),
    updated_at  TIMESTAMPTZ               DEFAULT NOW(),
    UNIQUE (sender_id, receiver_id)
);

CREATE TABLE friendships
(
    id        UUID PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
    user_id   UUID             NOT NULL,
    friend_id UUID             NOT NULL,
    created_at TIMESTAMPTZ              DEFAULT NOW(),
    UNIQUE (user_id, friend_id)
);
