ALTER TABLE follows DROP CONSTRAINT IF EXISTS follows_follower_id_board_name_key;
ALTER TABLE follows ALTER COLUMN board_name TYPE BIGINT USING 0;
ALTER TABLE follows RENAME COLUMN board_name TO followee_id;
ALTER TABLE follows ADD CONSTRAINT follows_follower_id_followee_id_key UNIQUE (follower_id, followee_id);
