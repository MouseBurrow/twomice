CREATE EXTENSION IF NOT EXISTS pgcrypto;

CREATE TABLE reports
(
    id          UUID PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
    reporter_id UUID             NOT NULL,
    target_type TEXT             NOT NULL,
    target_id   TEXT             NOT NULL,
    reason      TEXT             NOT NULL,
    created_at  TIMESTAMPTZ               DEFAULT NOW(),
    resolved    BOOL                      DEFAULT FALSE
);

CREATE TABLE moderation_actions
(
    id          UUID PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
    moderator_id UUID            NOT NULL,
    action_type TEXT              NOT NULL,
    target_id   TEXT              NOT NULL,
    reason      TEXT,
    created_at  TIMESTAMPTZ                DEFAULT NOW()
);
