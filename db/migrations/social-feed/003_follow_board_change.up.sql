-- 1. Alter follows table to use board_name instead of followee_id
ALTER TABLE follows RENAME COLUMN followee_id TO board_name;
ALTER TABLE follows ALTER COLUMN board_name TYPE TEXT;

-- 2. Recreate unique constraint
ALTER TABLE follows DROP CONSTRAINT IF EXISTS follows_follower_id_followee_id_key;
ALTER TABLE follows ADD CONSTRAINT follows_follower_id_board_name_key UNIQUE (follower_id, board_name);
