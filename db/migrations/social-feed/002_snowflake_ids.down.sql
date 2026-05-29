-- 1. Drop tables
DROP TABLE IF EXISTS feed_preferences CASCADE;
DROP TABLE IF EXISTS follows CASCADE;

-- 2. Drop snowflake helpers
DROP FUNCTION IF EXISTS snowflake_id();
DROP SEQUENCE IF EXISTS global_snowflake_seq;

-- 3. Recreate original UUID schema
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
    id          UUID PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
    user_id     UUID             NOT NULL UNIQUE,
    preferences JSONB           NOT NULL DEFAULT '{}'
);
