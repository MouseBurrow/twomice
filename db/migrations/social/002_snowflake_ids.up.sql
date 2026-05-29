-- 1. Drop tables
DROP TABLE IF EXISTS friendships CASCADE;
DROP TABLE IF EXISTS friend_requests CASCADE;

-- 2. Create Snowflake ID generation
CREATE SEQUENCE IF NOT EXISTS global_snowflake_seq;
CREATE OR REPLACE FUNCTION snowflake_id() RETURNS BIGINT AS $$
DECLARE
    epoch_ms CONSTANT BIGINT := 1735689600000;
    worker CONSTANT INT := 1;
    time_part BIGINT;
    seq_part BIGINT;
BEGIN
    time_part := (EXTRACT(EPOCH FROM clock_timestamp()) * 1000)::BIGINT - epoch_ms;
    seq_part := nextval('global_snowflake_seq')::BIGINT % 4096;
    RETURN (time_part << 22) | (worker << 12) | seq_part;
END;
$$ LANGUAGE plpgsql;

-- 3. Recreate tables with BIGINT
CREATE TABLE friend_requests
(
    id          BIGINT PRIMARY KEY NOT NULL DEFAULT snowflake_id(),
    sender_id   BIGINT             NOT NULL,
    receiver_id BIGINT             NOT NULL,
    status      TEXT             NOT NULL DEFAULT 'pending',
    created_at  TIMESTAMPTZ               DEFAULT NOW(),
    updated_at  TIMESTAMPTZ               DEFAULT NOW(),
    UNIQUE (sender_id, receiver_id)
);

CREATE TABLE friendships
(
    id         BIGINT PRIMARY KEY NOT NULL DEFAULT snowflake_id(),
    user_id    BIGINT             NOT NULL,
    friend_id  BIGINT             NOT NULL,
    created_at TIMESTAMPTZ              DEFAULT NOW(),
    UNIQUE (user_id, friend_id)
);
