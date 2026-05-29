-- 1. Drop tables
DROP TABLE IF EXISTS moderation_actions CASCADE;
DROP TABLE IF EXISTS reports CASCADE;

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
CREATE TABLE reports
(
    id          BIGINT PRIMARY KEY NOT NULL DEFAULT snowflake_id(),
    reporter_id BIGINT             NOT NULL,
    target_type TEXT             NOT NULL,
    target_id   TEXT             NOT NULL,
    reason      TEXT             NOT NULL,
    created_at  TIMESTAMPTZ               DEFAULT NOW(),
    resolved    BOOL                      DEFAULT FALSE
);

CREATE TABLE moderation_actions
(
    id           BIGINT PRIMARY KEY NOT NULL DEFAULT snowflake_id(),
    moderator_id BIGINT             NOT NULL,
    action_type  TEXT              NOT NULL,
    target_id    TEXT              NOT NULL,
    reason       TEXT,
    created_at   TIMESTAMPTZ                DEFAULT NOW()
);
