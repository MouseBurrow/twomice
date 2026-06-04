use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHasher,
};
use clap::{Parser, Subcommand};
use sqlx::postgres::PgPoolOptions;
use std::collections::HashMap;
use std::env;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};

#[derive(Parser)]
#[command(name = "twomice-db")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Run { service: String },
    Revert { service: String },
    Reset { service: String },
    Seed,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let cli = Cli::parse();

    match cli.command {
        Commands::Seed => seed().await?,
        Commands::Run { service } => migrate("run", &service)?,
        Commands::Revert { service } => migrate("revert", &service)?,
        Commands::Reset { service } => {
            reset(&service).await?;
            seed().await?;
        }
    }

    Ok(())
}

fn database_env_var(service: &str) -> String {
    match service {
        "social-feed" => "FEED_DATABASE_URL".into(),
        _ => format!("{}_DATABASE_URL", service.to_uppercase()),
    }
}

fn migrate(action: &str, service: &str) -> anyhow::Result<()> {
    let env_var = database_env_var(service);
    let database_url =
        env::var(&env_var).unwrap_or_else(|_| panic!("Environment variable {} not set", env_var));

    let migrations_dir = format!("db/migrations/{service}");

    let mut child = Command::new("sqlx")
        .args([
            "migrate",
            action,
            "--source",
            &migrations_dir,
            "--database-url",
            &database_url,
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    if let Some(stdout) = child.stdout.take() {
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            println!("{}", line?);
        }
    }

    if let Some(stderr) = child.stderr.take() {
        let reader = BufReader::new(stderr);
        for line in reader.lines() {
            eprintln!("{}", line?);
        }
    }

    let status = child.wait()?;
    if !status.success() {
        anyhow::bail!("Migration failed for service {}", service);
    }

    Ok(())
}

async fn reset(service: &str) -> anyhow::Result<()> {
    let env_var = database_env_var(service);
    let database_url =
        env::var(&env_var).unwrap_or_else(|_| panic!("Environment variable {} not set", env_var));

    let pool = PgPoolOptions::new().connect(&database_url).await?;
    sqlx::query("DROP SCHEMA IF EXISTS public CASCADE")
        .execute(&pool)
        .await?;
    sqlx::query("CREATE SCHEMA public").execute(&pool).await?;
    pool.close().await;

    println!("Schema dropped and recreated for {service}, running migrations...");
    migrate("run", service)
}

// ── base62 helpers (inlined from libs/utils) ──────────────────────────

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
    use rand::Rng;
    let mut rng = rand::thread_rng();
    (0..len)
        .map(|_| {
            let idx = rng.gen_range(0..B62_CHARS.len());
            B62_CHARS[idx] as char
        })
        .collect()
}

// ── seed ──────────────────────────────────────────────────────────────

async fn seed() -> anyhow::Result<()> {
    let auth_url = env::var("AUTH_DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://twomice:twomice@127.0.0.1:5432/auth".into());
    let post_url = env::var("POST_DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://twomice:twomice@127.0.0.1:5432/post".into());
    let moderation_url = env::var("MODERATION_DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://twomice:twomice@127.0.0.1:5432/moderation".into());
    let social_url = env::var("SOCIAL_DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://twomice:twomice@127.0.0.1:5432/social".into());
    let feed_url = env::var("FEED_DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://twomice:twomice@127.0.0.1:5432/social_feed".into());

    let auth_pool = PgPoolOptions::new().connect(&auth_url).await?;
    let post_pool = PgPoolOptions::new().connect(&post_url).await?;
    let moderation_pool = PgPoolOptions::new().connect(&moderation_url).await?;
    let social_pool = PgPoolOptions::new().connect(&social_url).await?;
    let feed_pool = PgPoolOptions::new().connect(&feed_url).await?;

    println!("── TwoMice — Seed Data ──────────────────────────");
    println!();
    println!("  DBs: auth={}  post={}", auth_url, post_url);
    println!("       moderation={}", moderation_url);
    println!("       social={}", social_url);
    println!("       feed={}", feed_url);
    println!();

    // ── Auth: create test users ────────────────────────────────────────
    let password = "testpass123";
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| anyhow::anyhow!("{}", e))?
        .to_string();

    let mut auth_new = 0;

    let mouse_id: i64 = sqlx::query_scalar(
        "INSERT INTO accounts (username, password_hash) VALUES ('mouse', $1)
         ON CONFLICT (username) DO UPDATE SET username = EXCLUDED.username
         RETURNING id",
    )
    .bind(&hash)
    .fetch_one(&auth_pool)
    .await?;
    auth_new += 1;

    let alice_id: i64 = sqlx::query_scalar(
        "INSERT INTO accounts (username, password_hash) VALUES ('alice', $1)
         ON CONFLICT (username) DO UPDATE SET username = EXCLUDED.username
         RETURNING id",
    )
    .bind(&hash)
    .fetch_one(&auth_pool)
    .await?;
    auth_new += 1;

    let bob_id: i64 = sqlx::query_scalar(
        "INSERT INTO accounts (username, password_hash) VALUES ('bob', $1)
         ON CONFLICT (username) DO UPDATE SET username = EXCLUDED.username
         RETURNING id",
    )
    .bind(&hash)
    .fetch_one(&auth_pool)
    .await?;
    auth_new += 1;

    println!("  [auth]       Seeded 3 users (new: {auth_new})");
    println!("               Credentials: mouse / alice / bob : {password}");
    println!();

    // ── Data definitions ───────────────────────────────────────────────

    let topic_list = [
        (
            "general",
            "General discussion about anything and everything mouse-related",
        ),
        (
            "cheese",
            "The finest cheeses from around the burrow. Camembert, Gouda, and beyond!",
        ),
        (
            "tech",
            "Bits, bytes, and tiny keyboards. Hardware and software for mice, by mice",
        ),
        (
            "art",
            "Show off your tiny paintings, squeak-ature drawings, and nest photography",
        ),
        (
            "gaming",
            "From Maze Runner to Cheese Heist — all things gaming",
        ),
        (
            "books",
            "Book club for well-read rodents. Reviews, recommendations, and literary chat",
        ),
    ];

    let post_defs = [
        ("general", "Welcome to TwoMice!", "Hey everyone! Welcome to TwoMice, the coziest corner of the internet. Grab some cheese and make yourself at home. Share your stories, ask questions, and don't forget to squeak hello!", mouse_id, None),
        ("general", "Forum rules and guidelines", "A few ground rules to keep our burrow friendly:\n\n1. Be kind to other mice\n2. No spam or advertising\n3. Keep discussions in the right boards\n4. Have fun!\n\nThat's it. We're pretty chill here.", alice_id, None),
        ("cheese", "Best cheddar I've ever nibbled", "Found this amazing aged cheddar at the farmer's market yesterday. Sharp, crumbly, with those perfect little crystals. 10/10 would nibble again. What's your favorite cheese discovery this month?", bob_id, None),
        ("cheese", "Gouda vs Edam: the ultimate showdown", "I've been going back and forth between these two Dutch classics. Gouda has that rich, caramel sweetness when aged, but Edam's nutty mildness is so versatile. Which side are you on?", mouse_id, None),
        ("tech", "Building my first mechanical keyboard", "Just ordered all the parts for my first custom mechanical keyboard build!\n\n- PCB: TinyType S (40%)\n- Switches: Kailh Box Jades (clicky!)\n- Keycaps: SA profile in earthy tones\n- Case: walnut wood\n\nWish me luck with the soldering!", alice_id, Some(vec!["keyboards", "hardware"])),
        ("tech", "Rust tip: using Result with axum", "Here's a quick pattern I've been using in my axum handlers:\n\nWrap your business logic in a service layer that returns Result<T, AppError>, then convert AppError into HTTP responses. Keeps your route handlers clean and testable.", mouse_id, Some(vec!["rust", "axum", "tutorial"])),
        ("art", "My latest watercolor: sunset over the wheat field", "Just finished this little painting! It's a view from the edge of the burrow looking out at the wheat field during golden hour. The wheat stalks are taller than I expected to paint. Posted a pic — feedback welcome!", bob_id, Some(vec!["watercolor", "illustration"])),
        ("art", "Drawn with cheese: edible art thread", "Has anyone else tried the nibble-and-draw technique? You basically sketch with different colored cheeses (cheddar orange, blue cheese veins, brie rind). The challenge is not eating your materials halfway through!", alice_id, Some(vec!["technique", "fun"])),
        ("gaming", "Cheese Heist speedrun world record broken!", "The legendary runner QuickMouse just beat Cheese Heist Any% in 42:13! The new strat uses a wall clip in the kitchen level to skip the entire cat section. Absolutely insane run.", mouse_id, Some(vec!["speedrun"])),
        ("gaming", "What are you playing this weekend?", "I'm diving back into Hollow Knight. Trying to finally beat the Pantheon of Hallownest. What's everyone else playing? Any hidden gem recommendations?", bob_id, None),
        ("books", "Just finished 'The Mouse and the Motorcycle'", "Re-reading a childhood classic hits different as an adult. Beverly Cleary really understood the mouse perspective — the adventure, the danger, the thrill of riding a toy motorcycle. Any other mouse-lit recommendations?", alice_id, None),
        ("books", "Building a cozy reading corner", "I've been working on my reading nook: a little cardboard box lined with shredded paper (the soft kind!), a tiny LED lamp, and a thimble of chamomile tea. Perfect for rainy day reading. Show me your reading setups!", mouse_id, None),
    ];

    let comment_defs: [(&str, &str, i64); 15] = [
        ("Welcome to TwoMice!", "So excited this place exists! Hello everyone!", bob_id),
        ("Welcome to TwoMice!", "Finally a place where I can talk about cheese without judgment.", alice_id),
        ("Forum rules and guidelines", "Good rules. Keeping it simple is the way to go.", mouse_id),
        ("Best cheddar I've ever nibbled", "Was it the clothbound kind? Those are always the best.", alice_id),
        ("Best cheddar I've ever nibbled", "You have to tell us which farmer's market! I need this cheese.", mouse_id),
        ("Best cheddar I've ever nibbled", "Cheddar with crystals is the peak of cheese evolution.", bob_id),
        ("Gouda vs Edam: the ultimate showdown", "Aged Gouda all the way. Those crunchy crystals are unbeatable.", bob_id),
        ("Gouda vs Edam: the ultimate showdown", "Hot take: smoked Gouda on a cracker with a tiny bit of honey.", alice_id),
        ("Building my first mechanical keyboard", "Box Jades are a bold choice for a first build! I respect it.", mouse_id),
        ("Building my first mechanical keyboard", "Please post the sound test when you're done! I love clicky switches.", bob_id),
        ("Rust tip: using Result with axum", "Great pattern. I do something similar but with thiserror for the error enum.", alice_id),
        ("My latest watercolor: sunset over the wheat field", "The way you captured the golden light is beautiful!", mouse_id),
        ("Drawn with cheese: edible art thread", "I tried this and ate my entire palette within 10 minutes.", bob_id),
        ("Cheese Heist speedrun world record broken!", "The kitchen clip is going to get patched for sure. Enjoy it while it lasts!", alice_id),
        ("Just finished 'The Mouse and the Motorcycle'", "Try Redwall next! Epic fantasy with mice, badgers, and the best feast descriptions ever.", bob_id),
    ];

    let reply_defs: [(usize, &str, i64); 6] = [
        (0, "Welcome aboard! This is going to be great.", mouse_id),
        (
            0,
            "Same here! Let's make this the best burrow on the net.",
            alice_id,
        ),
        (2, "Simple and fair. Good modding philosophy.", bob_id),
        (
            6,
            "Have you tried the 18-month aged Gouda? Life-changing.",
            mouse_id,
        ),
        (
            8,
            "The click is half the fun! Can't wait to hear it.",
            alice_id,
        ),
        (
            13,
            "They always patch the best strats. Speedrunners never get a break.",
            mouse_id,
        ),
    ];

    let post_vote_defs: [(&str, i64, i16); 14] = [
        ("Welcome to TwoMice!", alice_id, 1),
        ("Welcome to TwoMice!", bob_id, 1),
        ("Forum rules and guidelines", mouse_id, 1),
        ("Forum rules and guidelines", bob_id, 1),
        ("Best cheddar I've ever nibbled", mouse_id, 1),
        ("Best cheddar I've ever nibbled", alice_id, 1),
        ("Gouda vs Edam: the ultimate showdown", bob_id, 1),
        ("Gouda vs Edam: the ultimate showdown", alice_id, -1),
        ("Building my first mechanical keyboard", mouse_id, 1),
        ("Building my first mechanical keyboard", bob_id, 1),
        ("Rust tip: using Result with axum", alice_id, 1),
        ("Rust tip: using Result with axum", bob_id, 1),
        ("Cheese Heist speedrun world record broken!", bob_id, 1),
        ("Cheese Heist speedrun world record broken!", alice_id, -1),
    ];

    let comment_vote_defs: [(usize, i64, i16); 8] = [
        (4, alice_id, 1),
        (4, bob_id, 1),
        (6, mouse_id, 1),
        (7, mouse_id, 1),
        (10, bob_id, 1),
        (11, alice_id, 1),
        (14, alice_id, 1),
        (14, mouse_id, 1),
    ];

    // ── Post: create topics ─────────────────────────────────────────────
    struct PostRef {
        id: i64,
        slug: String,
    }
    struct CommentRef {
        id: i64,
        hash: String,
    }
    let mut post_map: HashMap<String, PostRef> = HashMap::new();
    let mut comment_refs: Vec<CommentRef> = Vec::new();
    let mut topic_ids: HashMap<String, i64> = HashMap::new();

    let mut topic_new = 0;
    for (name, description) in &topic_list {
        let existed: Option<i64> = sqlx::query_scalar("SELECT id FROM topics WHERE name = $1")
            .bind(name)
            .fetch_optional(&post_pool)
            .await?
            .flatten();
        let tid = if let Some(id) = existed {
            id
        } else {
            topic_new += 1;
            sqlx::query_scalar(
                "INSERT INTO topics (name, description) VALUES ($1, $2) RETURNING id",
            )
            .bind(name)
            .bind(description)
            .fetch_one(&post_pool)
            .await?
        };
        topic_ids.insert(name.to_string(), tid);
    }
    println!(
        "  [post]      Topics:     {}/{} new",
        topic_new,
        topic_list.len()
    );

    // ── Post: create posts ──────────────────────────────────────────────
    let mut post_new = 0;
    for (topic_name, title, content, creator_id, tags) in &post_defs {
        let topic_id = topic_ids[*topic_name];
        let existed: Option<(i64, String)> = sqlx::query_as(
            "SELECT id, slug FROM posts WHERE creator_id = $1 AND topic_id = $2 AND title = $3",
        )
        .bind(creator_id)
        .bind(topic_id)
        .bind(title)
        .fetch_optional(&post_pool)
        .await?;

        if let Some((post_id, slug)) = existed {
            post_map.insert(title.to_string(), PostRef { id: post_id, slug });
        } else {
            post_new += 1;
            let post_id: i64 = sqlx::query_scalar(
                "INSERT INTO posts (creator_id, topic_id, title, slug, content)
                 VALUES ($1, $2, $3, '', $4)
                 RETURNING id",
            )
            .bind(creator_id)
            .bind(topic_id)
            .bind(title)
            .bind(content)
            .fetch_one(&post_pool)
            .await?;

            let slug = encode_b62(post_id);
            sqlx::query("UPDATE posts SET slug = $1 WHERE id = $2")
                .bind(&slug)
                .bind(post_id)
                .execute(&post_pool)
                .await?;

            if let Some(tag_list) = tags {
                sqlx::query("UPDATE posts SET tags = $1 WHERE id = $2")
                    .bind(tag_list.as_slice())
                    .bind(post_id)
                    .execute(&post_pool)
                    .await?;
            }

            post_map.insert(title.to_string(), PostRef { id: post_id, slug });
        }
    }
    println!(
        "  [post]      Posts:      {}/{} new",
        post_new,
        post_defs.len()
    );

    // ── Post: create comments ───────────────────────────────────────────
    let mut comment_new = 0;
    for (post_title, content, sender_id) in &comment_defs {
        let post = post_map.get(*post_title).unwrap();
        let hash = random_b62(5);

        let result = sqlx::query(
            "INSERT INTO comments (hash, sender_id, post_id, content)
             VALUES ($1, $2, $3, $4)
             ON CONFLICT (post_id, hash) DO NOTHING",
        )
        .bind(&hash)
        .bind(sender_id)
        .bind(post.id)
        .bind(content)
        .execute(&post_pool)
        .await?;

        if result.rows_affected() > 0 {
            comment_new += 1;
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
        comment_defs.len()
    );

    // ── Post: create replies ────────────────────────────────────────────
    let mut reply_new = 0;
    for (comment_idx, content, sender_id) in &reply_defs {
        let comment = &comment_refs[*comment_idx];
        let hash = random_b62(5);

        let result = sqlx::query(
            "INSERT INTO replies (hash, sender_id, post_id, comment_id, content)
             VALUES ($1, $2, (SELECT post_id FROM comments WHERE id = $3), $3, $4)
             ON CONFLICT (post_id, hash) DO NOTHING",
        )
        .bind(&hash)
        .bind(sender_id)
        .bind(comment.id)
        .bind(content)
        .execute(&post_pool)
        .await?;

        if result.rows_affected() > 0 {
            reply_new += 1;
        }
    }
    println!(
        "  [post]      Replies:    {}/{} new",
        reply_new,
        reply_defs.len()
    );

    // ── Post: create votes ──────────────────────────────────────────────
    let mut pv_new = 0;
    for (post_title, user_id, direction) in &post_vote_defs {
        let post = post_map.get(*post_title).unwrap();
        let result = sqlx::query(
            "INSERT INTO post_votes (user_id, post_id, direction)
             VALUES ($1, $2, $3)
             ON CONFLICT (user_id, post_id) DO NOTHING",
        )
        .bind(user_id)
        .bind(post.id)
        .bind(direction)
        .execute(&post_pool)
        .await?;
        if result.rows_affected() > 0 {
            pv_new += 1;
        }
    }
    println!(
        "  [post]      Post votes: {}/{} new",
        pv_new,
        post_vote_defs.len()
    );

    let mut cv_new = 0;
    for (comment_idx, user_id, direction) in &comment_vote_defs {
        let comment = &comment_refs[*comment_idx];
        let result = sqlx::query(
            "INSERT INTO comment_votes (user_id, comment_id, direction)
             VALUES ($1, $2, $3)
             ON CONFLICT (user_id, comment_id) DO NOTHING",
        )
        .bind(user_id)
        .bind(comment.id)
        .bind(direction)
        .execute(&post_pool)
        .await?;
        if result.rows_affected() > 0 {
            cv_new += 1;
        }
    }
    println!(
        "  [post]      Cmt votes:  {}/{} new",
        cv_new,
        comment_vote_defs.len()
    );
    println!();

    let follow_defs: [(i64, &str); 7] = [
        (mouse_id, "general"),
        (mouse_id, "tech"),
        (mouse_id, "gaming"),
        (alice_id, "cheese"),
        (alice_id, "art"),
        (alice_id, "books"),
        (bob_id, "gaming"),
    ];

    let pref_defs: [(i64, &str); 3] = [
        (mouse_id, r#"{"sort":"hot","muted_boards":[]}"#),
        (alice_id, r#"{"sort":"new","muted_boards":[]}"#),
        (bob_id, r#"{"sort":"top","muted_boards":["art"]}"#),
    ];

    let friend_request_defs: [(i64, i64, &str); 3] = [
        (mouse_id, alice_id, "pending"),
        (alice_id, bob_id, "pending"),
        (bob_id, mouse_id, "accepted"),
    ];

    let friendship_defs: [(i64, i64); 2] = [(bob_id, mouse_id), (mouse_id, bob_id)];

    // ── Social Feed ─────────────────────────────────────────────────────
    let mut follow_new = 0;
    for (user_id, board_name) in &follow_defs {
        let result = sqlx::query(
            "INSERT INTO follows (follower_id, board_name)
             VALUES ($1, $2)
             ON CONFLICT (follower_id, board_name) DO NOTHING",
        )
        .bind(user_id)
        .bind(board_name)
        .execute(&feed_pool)
        .await?;
        if result.rows_affected() > 0 {
            follow_new += 1;
        }
    }
    println!(
        "  [feed]      Follows:        {}/{} new",
        follow_new,
        follow_defs.len()
    );

    let mut pref_new = 0;
    for (user_id, prefs_json) in &pref_defs {
        let result = sqlx::query(
            "INSERT INTO feed_preferences (user_id, preferences)
             VALUES ($1, $2::jsonb)
             ON CONFLICT (user_id) DO NOTHING",
        )
        .bind(user_id)
        .bind(prefs_json)
        .execute(&feed_pool)
        .await?;
        if result.rows_affected() > 0 {
            pref_new += 1;
        }
    }
    println!(
        "  [feed]      Preferences:    {}/{} new",
        pref_new,
        pref_defs.len()
    );
    println!();

    // ── Social ──────────────────────────────────────────────────────────
    let mut fr_new = 0;
    for (sender_id, receiver_id, status) in &friend_request_defs {
        let result = sqlx::query(
            "INSERT INTO friend_requests (sender_id, receiver_id, status)
             VALUES ($1, $2, $3)
             ON CONFLICT (sender_id, receiver_id) DO NOTHING",
        )
        .bind(sender_id)
        .bind(receiver_id)
        .bind(status)
        .execute(&social_pool)
        .await?;
        if result.rows_affected() > 0 {
            fr_new += 1;
        }
    }
    println!(
        "  [social]    Friend requests: {}/{} new",
        fr_new,
        friend_request_defs.len()
    );

    let mut fs_new = 0;
    for (user_id, friend_id) in &friendship_defs {
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
        friendship_defs.len()
    );
    println!();

    // ── Moderation ──────────────────────────────────────────────────────
    let mut report_new = 0;
    // Report 0: mouse reports "Best cheddar" post
    {
        let post = post_map.get("Best cheddar I've ever nibbled").unwrap();
        let existed: Option<i64> = sqlx::query_scalar(
            "SELECT id FROM reports WHERE reporter_id = $1 AND target_type = 'post' AND target_id = $2",
        )
        .bind(mouse_id)
        .bind(&post.slug)
        .fetch_optional(&moderation_pool)
        .await?
        .flatten();
        if existed.is_none() {
            sqlx::query(
                "INSERT INTO reports (reporter_id, target_type, target_id, reason, resolved)
                 VALUES ($1, 'post', $2, $3, false)",
            )
            .bind(mouse_id)
            .bind(&post.slug)
            .bind("Contains what looks like spam advertising for a cheese vendor")
            .execute(&moderation_pool)
            .await?;
            report_new += 1;
        }
    }
    // Report 1: alice reports comment 7
    {
        let comment = &comment_refs[7];
        let existed: Option<i64> = sqlx::query_scalar(
            "SELECT id FROM reports WHERE reporter_id = $1 AND target_type = 'comment' AND target_id = $2",
        )
        .bind(alice_id)
        .bind(&comment.hash)
        .fetch_optional(&moderation_pool)
        .await?
        .flatten();
        if existed.is_none() {
            sqlx::query(
                "INSERT INTO reports (reporter_id, target_type, target_id, reason, resolved)
                 VALUES ($1, 'comment', $2, $3, false)",
            )
            .bind(alice_id)
            .bind(&comment.hash)
            .bind("This comment uses harassing language about cheese preferences")
            .execute(&moderation_pool)
            .await?;
            report_new += 1;
        }
    }
    // Report 2: bob reports "Drawn with cheese" post (resolved)
    {
        let post = post_map
            .get("Drawn with cheese: edible art thread")
            .unwrap();
        let existed: Option<i64> = sqlx::query_scalar(
            "SELECT id FROM reports WHERE reporter_id = $1 AND target_type = 'post' AND target_id = $2",
        )
        .bind(bob_id)
        .bind(&post.slug)
        .fetch_optional(&moderation_pool)
        .await?
        .flatten();
        if existed.is_none() {
            sqlx::query(
                "INSERT INTO reports (reporter_id, target_type, target_id, reason, resolved)
                 VALUES ($1, 'post', $2, $3, true)",
            )
            .bind(bob_id)
            .bind(&post.slug)
            .bind("Off-topic — should this be in the art board?")
            .execute(&moderation_pool)
            .await?;
            report_new += 1;
        }
    }
    println!("  [mod]       Reports:          {}/3 new", report_new);

    let action_new;
    let post_slug = &post_map["Drawn with cheese: edible art thread"].slug;
    let existing_action: Option<i64> = sqlx::query_scalar(
        "SELECT id FROM moderation_actions WHERE action_type = 'warn' AND moderator_id = $1 AND target_id = $2",
    )
    .bind(bob_id)
    .bind(post_slug)
    .fetch_optional(&moderation_pool)
    .await?
    .flatten();
    if existing_action.is_none() {
        sqlx::query(
            "INSERT INTO moderation_actions (moderator_id, action_type, target_id, reason)
             VALUES ($1, 'warn', $2, 'Post moved to correct board — please keep topics in their designated boards')",
        )
        .bind(bob_id)
        .bind(post_slug)
        .execute(&moderation_pool)
        .await?;
        action_new = 1;
    } else {
        action_new = 0;
    }
    println!("  [mod]       Actions:          {}/1 new", action_new);
    println!();

    // ── Summary ─────────────────────────────────────────────────────────
    println!("── Seed complete ────────────────────────────────");
    println!();
    println!("  auth:        3 users");
    println!(
        "  post:        {} topics, {} posts, {} comments",
        topic_list.len(),
        post_defs.len(),
        comment_defs.len()
    );
    println!(
        "               {} replies, {} post votes, {} cmt votes",
        reply_defs.len(),
        post_vote_defs.len(),
        comment_vote_defs.len()
    );
    println!(
        "  feed:        {} follows, {} preferences",
        follow_defs.len(),
        pref_defs.len()
    );
    println!(
        "  social:      {} friend requests, {} friendships",
        friend_request_defs.len(),
        friendship_defs.len()
    );
    println!("  moderation:  3 reports, 1 action");
    println!();
    println!("  Login: mouse / alice / bob : {password}");

    Ok(())
}
