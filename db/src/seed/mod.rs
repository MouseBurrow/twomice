mod data;

use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHasher,
};
use rand::Rng;
use sqlx::postgres::PgPoolOptions;
use std::collections::HashMap;
use std::env;

const B62_CHARS: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

fn encode_b62(value: i64) -> String {
    if value == 0 {
        return "0".to_string();
    }
    let mut positive = value.unsigned_abs();
    let mut chars = Vec::new();
    while positive > 0 {
        let idx = (positive % 62) as usize;
        chars.push(B62_CHARS[idx] as char);
        positive /= 62;
    }
    if value < 0 {
        chars.push('-');
    }
    chars.reverse();
    chars.into_iter().collect()
}

fn random_b62(len: usize) -> String {
    let mut rng = rand::thread_rng();
    (0..len)
        .map(|_| {
            let idx = rng.gen_range(0..B62_CHARS.len());
            B62_CHARS[idx] as char
        })
        .collect()
}

fn db_url(env_var: &str, default: &str) -> String {
    env::var(env_var).unwrap_or_else(|_| default.into())
}


pub async fn seed() -> anyhow::Result<()> {
    let auth_url = db_url(
        "AUTH_DATABASE_URL",
        "postgresql://twomice:twomice@127.0.0.1:5432/auth",
    );
    let post_url = db_url(
        "POST_DATABASE_URL",
        "postgresql://twomice:twomice@127.0.0.1:5432/post",
    );
    let moderation_url = db_url(
        "MODERATION_DATABASE_URL",
        "postgresql://twomice:twomice@127.0.0.1:5432/moderation",
    );
    let social_url = db_url(
        "SOCIAL_DATABASE_URL",
        "postgresql://twomice:twomice@127.0.0.1:5432/social",
    );
    let feed_url = db_url(
        "FEED_DATABASE_URL",
        "postgresql://twomice:twomice@127.0.0.1:5432/social_feed",
    );

    let auth_pool = PgPoolOptions::new().connect(&auth_url).await?;
    let post_pool = PgPoolOptions::new().connect(&post_url).await?;
    let moderation_pool = PgPoolOptions::new().connect(&moderation_url).await?;
    let social_pool = PgPoolOptions::new().connect(&social_url).await?;
    let feed_pool = PgPoolOptions::new().connect(&feed_url).await?;

    println!("── TwoMice — Seed Data ──────────────────────────");
    println!();
    println!(
        "  DBs: auth={}  post={}",
        auth_url, post_url
    );
    println!("       moderation={}", moderation_url);
    println!("       social={}", social_url);
    println!("       feed={}", feed_url);
    println!();

    // ── Auth: create topic_tags table (dev bootstrap) ────────────────
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS topic_tags (
            topic_id BIGINT NOT NULL REFERENCES topics(id) ON DELETE CASCADE,
            tag_name TEXT NOT NULL,
            PRIMARY KEY (topic_id, tag_name)
        )",
    )
    .execute(&post_pool)
    .await?;

    // ── Auth: hash password once ─────────────────────────────────────
    let password = "testpass123";
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| anyhow::anyhow!("{}", e))?
        .to_string();

    let mut auth_new = 0;
    let mut user_ids: HashMap<&'static str, i64> = HashMap::new();

    for user in data::USERS {
        let id: i64 = sqlx::query_scalar(
            "INSERT INTO accounts (username, password_hash, is_admin)
             VALUES ($1, $2, $3)
             ON CONFLICT (username) DO UPDATE SET username = EXCLUDED.username
             RETURNING id",
        )
        .bind(user.username)
        .bind(&hash)
        .bind(user.is_admin)
        .fetch_one(&auth_pool)
        .await?;
        user_ids.insert(user.username, id);
        auth_new += 1;
    }
    println!("  [auth]       Seeded {} users (new: {})", data::USERS.len(), auth_new);
    println!("               Credentials: any username : {}", password);
    println!();

    // ── Auth: sessions ──────────────────────────────────────────────
    let mut sess_new = 0;
    for sess in data::SESSIONS {
        if let Some(&uid) = user_ids.get(sess.username) {
            let exists: Option<i64> = sqlx::query_scalar(
                "SELECT id FROM sessions WHERE account_id = $1",
            )
            .bind(uid)
            .fetch_optional(&auth_pool)
            .await?
            .flatten();
            if exists.is_none() {
                let token = random_b62(32);
                sqlx::query(
                    "INSERT INTO sessions (account_id, session_token)
                     VALUES ($1, $2)",
                )
                .bind(uid)
                .bind(&token)
                .execute(&auth_pool)
                .await?;
                sess_new += 1;
            }
        }
    }
    println!("  [auth]       Sessions:   {}/{} new", sess_new, data::SESSIONS.len());
    println!();

    // ── Post: topics + topic_tags ────────────────────────────────────
    let mut topic_ids: HashMap<&'static str, i64> = HashMap::new();

    for topic in data::TOPICS {
        let tid: i64 = sqlx::query_scalar(
            "INSERT INTO topics (name, description)
             VALUES ($1, $2)
             ON CONFLICT (name) DO UPDATE SET description = EXCLUDED.description
             RETURNING id",
        )
        .bind(topic.name)
        .bind(topic.description)
        .fetch_one(&post_pool)
        .await?;

        topic_ids.insert(topic.name, tid);

        for tag in topic.allowed_tags {
            sqlx::query(
                "INSERT INTO topic_tags (topic_id, tag_name)
                 VALUES ($1, $2)
                 ON CONFLICT DO NOTHING",
            )
            .bind(tid)
            .bind(tag)
            .execute(&post_pool)
            .await?;
        }
    }

    println!(
        "  [post]      Topics:     {}/{} new",
        data::TOPICS.len(),
        data::TOPICS.len()
    );

    // ── Post: posts ─────────────────────────────────────────────────
    struct PostRef {
        id: i64,
        slug: String,
    }
    struct CommentRef {
        id: i64,
        hash: String,
    }
    struct ReplyRef {
        id: i64,
    }

    let mut post_map: HashMap<&'static str, PostRef> = HashMap::new();
    let mut comment_refs: Vec<CommentRef> = Vec::new();
    let mut reply_refs: Vec<ReplyRef> = Vec::new();
    let mut post_new = 0;

    for post in data::POSTS {
        let creator_id = user_ids.get(post.creator).unwrap();
        let topic_id = topic_ids.get(post.topic).unwrap();

        let existed: Option<(i64, String)> = sqlx::query_as(
            "SELECT id, slug FROM posts WHERE creator_id = $1 AND topic_id = $2 AND title = $3",
        )
        .bind(creator_id)
        .bind(topic_id)
        .bind(post.title)
        .fetch_optional(&post_pool)
        .await?;

        if let Some((post_id, slug)) = existed {
            post_map.insert(post.title, PostRef { id: post_id, slug });
            // Update view_count and tags on re-run
            if post.view_count > 0 {
                sqlx::query("UPDATE posts SET view_count = GREATEST(view_count, $1) WHERE id = $2")
                    .bind(post.view_count as i64)
                    .bind(post_id)
                    .execute(&post_pool)
                    .await?;
            }
            if !post.tags.is_empty() {
                sqlx::query("UPDATE posts SET tags = $1 WHERE id = $2 AND tags = '{}'")
                    .bind(post.tags)
                    .bind(post_id)
                    .execute(&post_pool)
                    .await?;
            }
        } else {
            post_new += 1;
            let post_id: i64 = sqlx::query_scalar(
                "INSERT INTO posts (creator_id, topic_id, title, slug, content, image_url)
                 VALUES ($1, $2, $3, '', $4, $5)
                 RETURNING id",
            )
            .bind(creator_id)
            .bind(topic_id)
            .bind(post.title)
            .bind(post.content)
            .bind(post.image_url)
            .fetch_one(&post_pool)
            .await?;

            let slug = encode_b62(post_id);
            sqlx::query("UPDATE posts SET slug = $1 WHERE id = $2")
                .bind(&slug)
                .bind(post_id)
                .execute(&post_pool)
                .await?;

            // Set tags
            if !post.tags.is_empty() {
                sqlx::query("UPDATE posts SET tags = $1 WHERE id = $2")
                    .bind(post.tags)
                    .bind(post_id)
                    .execute(&post_pool)
                    .await?;
            }

            // Set view_count
            if post.view_count > 0 {
                sqlx::query("UPDATE posts SET view_count = $1 WHERE id = $2")
                    .bind(post.view_count as i64)
                    .bind(post_id)
                    .execute(&post_pool)
                    .await?;
            }

            // Set created_at to days_ago
            sqlx::query("UPDATE posts SET created_at = NOW() - $1 * INTERVAL '1 day' WHERE id = $2")
                .bind(post.days_ago as i32)
                .bind(post_id)
                .execute(&post_pool)
                .await?;

            post_map.insert(
                post.title,
                PostRef {
                    id: post_id,
                    slug,
                },
            );
        }
    }
    println!(
        "  [post]      Posts:      {}/{} new",
        post_new,
        data::POSTS.len()
    );

    // ── Post: comments ──────────────────────────────────────────────
    let mut comment_new = 0;

    for comment in data::COMMENTS {
        let post = post_map.get(comment.post_title).unwrap();
        let sender_id = user_ids.get(comment.sender).unwrap();
        let hash = random_b62(5);

        let result = sqlx::query(
            "INSERT INTO comments (hash, sender_id, post_id, content)
             VALUES ($1, $2, $3, $4)
             ON CONFLICT (post_id, hash) DO NOTHING",
        )
        .bind(&hash)
        .bind(sender_id)
        .bind(post.id)
        .bind(comment.content)
        .execute(&post_pool)
        .await?;

        if result.rows_affected() > 0 {
            comment_new += 1;
            // Set created_at to days_ago
            sqlx::query("UPDATE comments SET created_at = NOW() - $1 * INTERVAL '1 day' WHERE hash = $2 AND post_id = $3")
                .bind(comment.days_ago as i32)
                .bind(&hash)
                .bind(post.id)
                .execute(&post_pool)
                .await?;
        }

        let comment_id: i64 =
            sqlx::query_scalar("SELECT id FROM comments WHERE hash = $1 AND post_id = $2")
                .bind(&hash)
                .bind(post.id)
                .fetch_one(&post_pool)
                .await?;

        comment_refs.push(CommentRef {
            id: comment_id,
            hash,
        });
    }
    println!(
        "  [post]      Comments:   {}/{} new",
        comment_new,
        data::COMMENTS.len()
    );

    // ── Post: replies (with nesting) ────────────────────────────────
    let mut reply_new = 0;

    for reply in data::REPLIES {
        let post = post_map.get(reply.post_title).unwrap();
        let comment = &comment_refs[reply.comment_idx];
        let sender_id = user_ids.get(reply.sender).unwrap();
        let hash = random_b62(5);

        let parent_reply_id: Option<i64> = reply.parent_reply_idx.map(|idx| reply_refs[idx].id);

        let result = if let Some(pr_id) = parent_reply_id {
            sqlx::query(
                "INSERT INTO replies (hash, sender_id, post_id, comment_id, reply_id, content)
                 VALUES ($1, $2, $3, $4, $5, $6)
                 ON CONFLICT (post_id, hash) DO NOTHING",
            )
            .bind(&hash)
            .bind(sender_id)
            .bind(post.id)
            .bind(comment.id)
            .bind(pr_id)
            .bind(reply.content)
            .execute(&post_pool)
            .await?
        } else {
            sqlx::query(
                "INSERT INTO replies (hash, sender_id, post_id, comment_id, content)
                 VALUES ($1, $2, $3, $4, $5)
                 ON CONFLICT (post_id, hash) DO NOTHING",
            )
            .bind(&hash)
            .bind(sender_id)
            .bind(post.id)
            .bind(comment.id)
            .bind(reply.content)
            .execute(&post_pool)
            .await?
        };

        if result.rows_affected() > 0 {
            reply_new += 1;
            // Set created_at to days_ago
            sqlx::query("UPDATE replies SET created_at = NOW() - $1 * INTERVAL '1 day' WHERE hash = $2 AND post_id = $3")
                .bind(reply.days_ago as i32)
                .bind(&hash)
                .bind(post.id)
                .execute(&post_pool)
                .await?;
        }

        let reply_id: i64 =
            sqlx::query_scalar("SELECT id FROM replies WHERE hash = $1 AND post_id = $2")
                .bind(&hash)
                .bind(post.id)
                .fetch_one(&post_pool)
                .await?;

        reply_refs.push(ReplyRef { id: reply_id });
    }
    println!(
        "  [post]      Replies:    {}/{} new",
        reply_new,
        data::REPLIES.len()
    );

    // ── Post: post votes ────────────────────────────────────────────
    let mut pv_new = 0;
    for vote in data::POST_VOTES {
        let post = post_map.get(vote.post_title).unwrap();
        let user_id = user_ids.get(vote.voter).unwrap();
        let result = sqlx::query(
            "INSERT INTO post_votes (user_id, post_id, direction)
             VALUES ($1, $2, $3)
             ON CONFLICT (user_id, post_id) DO NOTHING",
        )
        .bind(user_id)
        .bind(post.id)
        .bind(vote.direction)
        .execute(&post_pool)
        .await?;
        if result.rows_affected() > 0 {
            pv_new += 1;
        }
    }
    println!(
        "  [post]      Post votes: {}/{} new",
        pv_new,
        data::POST_VOTES.len()
    );

    // ── Post: comment votes ──────────────────────────────────────────
    let mut cv_new = 0;
    for vote in data::COMMENT_VOTES {
        let comment = &comment_refs[vote.comment_idx];
        let user_id = user_ids.get(vote.voter).unwrap();
        let result = sqlx::query(
            "INSERT INTO comment_votes (user_id, comment_id, direction)
             VALUES ($1, $2, $3)
             ON CONFLICT (user_id, comment_id) DO NOTHING",
        )
        .bind(user_id)
        .bind(comment.id)
        .bind(vote.direction)
        .execute(&post_pool)
        .await?;
        if result.rows_affected() > 0 {
            cv_new += 1;
        }
    }
    println!(
        "  [post]      Cmt votes:  {}/{} new",
        cv_new,
        data::COMMENT_VOTES.len()
    );
    println!();

    // ── Social feed: follows ────────────────────────────────────────
    let mut follow_new = 0;
    for follow in data::FOLLOWS {
        let follower_id = user_ids.get(follow.follower).unwrap();
        let result = sqlx::query(
            "INSERT INTO follows (follower_id, board_name)
             VALUES ($1, $2)
             ON CONFLICT (follower_id, board_name) DO NOTHING",
        )
        .bind(follower_id)
        .bind(follow.board_name)
        .execute(&feed_pool)
        .await?;
        if result.rows_affected() > 0 {
            follow_new += 1;
        }
    }
    println!(
        "  [feed]      Follows:        {}/{} new",
        follow_new,
        data::FOLLOWS.len()
    );

    // ── Social feed: preferences ─────────────────────────────────────
    let mut pref_new = 0;
    for pref in data::PREFERENCES {
        let user_id = user_ids.get(pref.user).unwrap();
        let prefs_json = serde_json::json!({
            "sort": pref.sort,
            "muted_boards": pref.muted_boards,
        });
        let result = sqlx::query(
            "INSERT INTO feed_preferences (user_id, preferences)
             VALUES ($1, $2::jsonb)
             ON CONFLICT (user_id) DO NOTHING",
        )
        .bind(user_id)
        .bind(&prefs_json.to_string())
        .execute(&feed_pool)
        .await?;
        if result.rows_affected() > 0 {
            pref_new += 1;
        }
    }
    println!(
        "  [feed]      Preferences:    {}/{} new",
        pref_new,
        data::PREFERENCES.len()
    );
    println!();

    // ── Social: friend requests ──────────────────────────────────────
    let mut fr_new = 0;
    for fr in data::FRIEND_REQUESTS {
        let sender_id = user_ids.get(fr.sender).unwrap();
        let receiver_id = user_ids.get(fr.receiver).unwrap();
        let result = sqlx::query(
            "INSERT INTO friend_requests (sender_id, receiver_id, status)
             VALUES ($1, $2, $3)
             ON CONFLICT (sender_id, receiver_id) DO NOTHING",
        )
        .bind(sender_id)
        .bind(receiver_id)
        .bind(fr.status)
        .execute(&social_pool)
        .await?;
        if result.rows_affected() > 0 {
            fr_new += 1;
        }
    }
    println!(
        "  [social]    Friend requests: {}/{} new",
        fr_new,
        data::FRIEND_REQUESTS.len()
    );

    // ── Social: friendships ──────────────────────────────────────────
    let mut fs_new = 0;
    for fs in data::FRIENDSHIPS {
        let user_id = user_ids.get(fs.user).unwrap();
        let friend_id = user_ids.get(fs.friend).unwrap();
        let result = sqlx::query(
            "INSERT INTO friendships (user_id, friend_id)
             VALUES ($1, $2)
             ON CONFLICT (user_id, friend_id) DO NOTHING",
        )
        .bind(user_id)
        .bind(friend_id)
        .execute(&social_pool)
        .await?;
        if result.rows_affected() > 0 {
            fs_new += 1;
        }
    }
    println!(
        "  [social]    Friendships:     {}/{} new",
        fs_new,
        data::FRIENDSHIPS.len()
    );
    println!();

    // ── Moderation: reports ─────────────────────────────────────────
    let mut report_new = 0;
    for report in data::REPORTS {
        let reporter_id = user_ids.get(report.reporter).unwrap();
        let target_id = if report.target_type == "comment" {
            if let Some(ci) = report.target_comment_idx {
                comment_refs[ci].hash.clone()
            } else {
                continue;
            }
        } else {
            if let Some(post) = post_map.get(report.target_post_title) {
                post.slug.clone()
            } else {
                continue;
            }
        };

        let existed: Option<i64> = sqlx::query_scalar(
            "SELECT id FROM reports WHERE reporter_id = $1 AND target_type = $2 AND target_id = $3",
        )
        .bind(reporter_id)
        .bind(report.target_type)
        .bind(&target_id)
        .fetch_optional(&moderation_pool)
        .await?
        .flatten();

        if existed.is_none() {
            sqlx::query(
                "INSERT INTO reports (reporter_id, target_type, target_id, reason, resolved)
                 VALUES ($1, $2, $3, $4, $5)",
            )
            .bind(reporter_id)
            .bind(report.target_type)
            .bind(&target_id)
            .bind(report.reason)
            .bind(report.resolved)
            .execute(&moderation_pool)
            .await?;
            report_new += 1;
        }
    }
    println!(
        "  [mod]       Reports:          {}/{} new",
        report_new,
        data::REPORTS.len()
    );

    // ── Moderation: actions ─────────────────────────────────────────
    let mut action_new = 0;
    for action in data::MOD_ACTIONS {
        let moderator_id = user_ids.get(action.moderator).unwrap();
        if let Some(post) = post_map.get(action.target_post_title) {
            let target_id = &post.slug;
            let existed: Option<i64> = sqlx::query_scalar(
                "SELECT id FROM moderation_actions WHERE action_type = $1 AND moderator_id = $2 AND target_id = $3",
            )
            .bind(action.action_type)
            .bind(moderator_id)
            .bind(target_id)
            .fetch_optional(&moderation_pool)
            .await?
            .flatten();

            if existed.is_none() {
                sqlx::query(
                    "INSERT INTO moderation_actions (moderator_id, action_type, target_id, reason)
                     VALUES ($1, $2, $3, $4)",
                )
                .bind(moderator_id)
                .bind(action.action_type)
                .bind(target_id)
                .bind(action.reason)
                .execute(&moderation_pool)
                .await?;
                action_new += 1;
            }
        }
    }
    println!(
        "  [mod]       Actions:          {}/{} new",
        action_new,
        data::MOD_ACTIONS.len()
    );
    println!();

    // ── Summary ──────────────────────────────────────────────────────
    println!("── Seed complete ────────────────────────────────");
    println!();
    println!("  auth:        {} users", data::USERS.len());
    println!(
        "  post:        {} topics, {} posts, {} comments",
        data::TOPICS.len(),
        data::POSTS.len(),
        data::COMMENTS.len()
    );
    println!(
        "               {} replies, {} post votes, {} cmt votes",
        data::REPLIES.len(),
        data::POST_VOTES.len(),
        data::COMMENT_VOTES.len()
    );
    println!(
        "  feed:        {} follows, {} preferences",
        data::FOLLOWS.len(),
        data::PREFERENCES.len()
    );
    println!(
        "  social:      {} friend requests, {} friendships",
        data::FRIEND_REQUESTS.len(),
        data::FRIENDSHIPS.len()
    );
    println!(
        "  moderation:  {} reports, {} actions",
        data::REPORTS.len(),
        data::MOD_ACTIONS.len()
    );
    println!();
    println!("  Login: any username : {}", password);

    Ok(())
}
