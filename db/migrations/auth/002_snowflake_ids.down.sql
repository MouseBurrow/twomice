-- 1. Drop stored procedures
DROP FUNCTION IF EXISTS create_account(TEXT, TEXT);
DROP FUNCTION IF EXISTS get_password_hash(TEXT);
DROP FUNCTION IF EXISTS create_session(BIGINT);
DROP FUNCTION IF EXISTS logout_session(TEXT);
DROP FUNCTION IF EXISTS validate_token(TEXT);
DROP FUNCTION IF EXISTS get_account_info(BIGINT);

-- 2. Drop tables
DROP TABLE IF EXISTS sessions CASCADE;
DROP TABLE IF EXISTS accounts CASCADE;

-- 3. Drop snowflake helpers
DROP FUNCTION IF EXISTS snowflake_id();
DROP SEQUENCE IF EXISTS global_snowflake_seq;

-- 4. Recreate original UUID schema
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

-- 5. Recreate original stored procedures with UUID
CREATE OR REPLACE FUNCTION create_account(
    p_username TEXT,
    p_password_hash TEXT
)
    RETURNS TEXT
    LANGUAGE plpgsql
AS
$$
DECLARE
    new_account_id    UUID;
    new_session_token TEXT;
BEGIN
    INSERT INTO accounts (username, password_hash)
    VALUES (p_username, p_password_hash)
    RETURNING id INTO new_account_id;

    SELECT encode(gen_random_bytes(32), 'hex') INTO new_session_token;

    INSERT INTO sessions (account_id, session_token)
    VALUES (new_account_id, new_session_token);

    RETURN new_session_token;
EXCEPTION
    WHEN unique_violation THEN
        RAISE EXCEPTION 'Username already exists' USING ERRCODE = '23505';
END;
$$;

CREATE OR REPLACE FUNCTION get_password_hash(p_username TEXT) RETURNS TEXT
    LANGUAGE plpgsql AS
$$
DECLARE
    stored_hash TEXT;
BEGIN
    SELECT password_hash
    INTO stored_hash
    FROM accounts
    WHERE username = p_username;

    IF NOT FOUND THEN
        RAISE EXCEPTION 'Invalid username' USING ERRCODE = 'P0000';
    END IF;

    RETURN stored_hash;
END;
$$;

CREATE OR REPLACE FUNCTION create_session(p_account_id UUID)
    RETURNS TEXT
    LANGUAGE plpgsql
AS
$$
DECLARE
    new_token TEXT;
BEGIN
    SELECT encode(gen_random_bytes(32), 'hex') INTO new_token;

    INSERT INTO sessions (account_id, session_token)
    VALUES (p_account_id, new_token);

    RETURN new_token;
END;
$$;

CREATE OR REPLACE FUNCTION logout_session(p_session_token TEXT)
    RETURNS VOID
    LANGUAGE plpgsql
AS
$$
BEGIN
    DELETE
    FROM sessions
    WHERE session_token = p_session_token;

    IF NOT FOUND THEN
        RAISE EXCEPTION 'Session not found'
            USING ERRCODE = 'P0002';
    END IF;
END;
$$;

CREATE OR REPLACE FUNCTION validate_token(p_session_token TEXT)
    RETURNS UUID
    LANGUAGE plpgsql
AS
$$
DECLARE
    existing_account_id UUID;
BEGIN
    SELECT account_id
    INTO existing_account_id
    FROM sessions
    WHERE session_token = p_session_token
      AND expires_at > NOW();

    IF NOT FOUND THEN
        RETURN NULL;
    END IF;

    UPDATE sessions
    SET last_used_at = NOW(),
        expires_at   = (NOW() + INTERVAL '30 days')
    WHERE session_token = p_session_token;

    RETURN existing_account_id;
END;
$$;

CREATE OR REPLACE FUNCTION get_account_info(p_user_id UUID)
    RETURNS TABLE
            (
                username   VARCHAR(50),
                is_admin   BOOLEAN,
                created_at TIMESTAMPTZ,
                updated_at TIMESTAMPTZ
            )
    LANGUAGE plpgsql
AS
$$
BEGIN
    IF NOT EXISTS (SELECT 1
                   FROM accounts
                   WHERE id = p_user_id) THEN
        RAISE EXCEPTION 'Account not found'
            USING ERRCODE = 'P0001';
    END IF;

    RETURN QUERY SELECT a.username, a.is_admin, a.created_at, a.updated_at FROM accounts as a WHERE a.id = p_user_id;
END;
$$;
