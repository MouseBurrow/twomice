CREATE EXTENSION IF NOT EXISTS pgcrypto;

CREATE TABLE follows
(
    id          UUID PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
    follower_id UUID             NOT NULL,
    followee_id UUID             NOT NULL,
    created_at  TIMESTAMPTZ               DEFAULT NOW(),
    UNIQUE (follower_id, followee_id)
);

CREATE TABLE feed_preferences
(
    id         UUID PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
    user_id    UUID             NOT NULL UNIQUE,
    preferences JSONB           NOT NULL DEFAULT '{}'
);
