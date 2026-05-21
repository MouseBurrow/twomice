use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHasher,
};
use clap::{Parser, Subcommand};
use sqlx::postgres::PgPoolOptions;
use sqlx::types::Uuid;
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
    sqlx::query("CREATE SCHEMA public")
        .execute(&pool)
        .await?;
    pool.close().await;

    println!("Schema dropped and recreated for {service}, running migrations...");
    migrate("run", service)
}

async fn seed() -> anyhow::Result<()> {
    let auth_url = env::var("AUTH_DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://twomice:twomice@127.0.0.1:5432/auth".into());
    let post_url = env::var("POST_DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://twomice:twomice@127.0.0.1:5432/post".into());

    let auth_pool = PgPoolOptions::new().connect(&auth_url).await?;
    let post_pool = PgPoolOptions::new().connect(&post_url).await?;

    let existing: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM accounts WHERE username IN ('mouse', 'alice', 'bob')",
    )
    .fetch_one(&auth_pool)
    .await?;
    if existing.0 > 0 {
        println!("Seed data already exists, skipping.");
        return Ok(());
    }

    // ── Auth: create test users ────────────────────────────────────────
    let password = "testpass123";
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| anyhow::anyhow!("{}", e))?
        .to_string();

    let test_user_id: (Uuid,) = sqlx::query_as(
        "INSERT INTO accounts (username, password_hash) VALUES ('mouse', $1)
         ON CONFLICT (username) DO NOTHING
         RETURNING id",
    )
    .bind(&hash)
    .fetch_one(&auth_pool)
    .await?;

    let alice_id: (Uuid,) = sqlx::query_as(
        "INSERT INTO accounts (username, password_hash) VALUES ('alice', $1)
         ON CONFLICT (username) DO NOTHING
         RETURNING id",
    )
    .bind(&hash)
    .fetch_one(&auth_pool)
    .await?;

    let bob_id: (Uuid,) = sqlx::query_as(
        "INSERT INTO accounts (username, password_hash) VALUES ('bob', $1)
         ON CONFLICT (username) DO NOTHING
         RETURNING id",
    )
    .bind(&hash)
    .fetch_one(&auth_pool)
    .await?;

    println!("Seeded users: mouse, alice, bob (password: {password})");

    // ── Post: create topics ─────────────────────────────────────────────
    let topics = [
        ("general", "General discussion about anything and everything mouse-related"),
        ("cheese", "The finest cheeses from around the burrow. Camembert, Gouda, and beyond!"),
        ("tech", "Bits, bytes, and tiny keyboards. Hardware and software for mice, by mice"),
        ("art", "Show off your tiny paintings, squeak-ature drawings, and nest photography"),
        ("gaming", "From Maze Runner to Cheese Heist — all things gaming"),
        ("books", "Book club for well-read rodents. Reviews, recommendations, and literary chat"),
    ];

    for (name, description) in &topics {
        sqlx::query(
            "INSERT INTO topics (name, description) VALUES ($1, $2)
             ON CONFLICT (name) DO NOTHING",
        )
        .bind(name)
        .bind(description)
        .execute(&post_pool)
        .await?;
    }
    println!("Seeded {} topics", topics.len());

    // ── Post: create posts ──────────────────────────────────────────────
    let posts = [
        ("general", "Welcome to TwoMice!", "Hey everyone! Welcome to TwoMice, the coziest corner of the internet. Grab some cheese and make yourself at home. Share your stories, ask questions, and don't forget to squeak hello!", &test_user_id.0),
        ("general", "Forum rules and guidelines", "A few ground rules to keep our burrow friendly:\n\n1. Be kind to other mice\n2. No spam or advertising\n3. Keep discussions in the right boards\n4. Have fun!\n\nThat's it. We're pretty chill here.", &alice_id.0),
        ("cheese", "Best cheddar I've ever nibbled", "Found this amazing aged cheddar at the farmer's market yesterday. Sharp, crumbly, with those perfect little crystals. 10/10 would nibble again. What's your favorite cheese discovery this month?", &bob_id.0),
        ("cheese", "Gouda vs Edam: the ultimate showdown", "I've been going back and forth between these two Dutch classics. Gouda has that rich, caramel sweetness when aged, but Edam's nutty mildness is so versatile. Which side are you on?", &test_user_id.0),
        ("tech", "Building my first mechanical keyboard", "Just ordered all the parts for my first custom mechanical keyboard build!\n\n- PCB: TinyType S (40%)\n- Switches: Kailh Box Jades (clicky!)\n- Keycaps: SA profile in earthy tones\n- Case: walnut wood\n\nWish me luck with the soldering!", &alice_id.0),
        ("tech", "Rust tip: using Result with axum", "Here's a quick pattern I've been using in my axum handlers:\n\nWrap your business logic in a service layer that returns Result<T, AppError>, then convert AppError into HTTP responses. Keeps your route handlers clean and testable.", &test_user_id.0),
        ("art", "My latest watercolor: sunset over the wheat field", "Just finished this little painting! It's a view from the edge of the burrow looking out at the wheat field during golden hour. The wheat stalks are taller than I expected to paint. Posted a pic — feedback welcome!", &bob_id.0),
        ("art", "Drawn with cheese: edible art thread", "Has anyone else tried the nibble-and-draw technique? You basically sketch with different colored cheeses (cheddar orange, blue cheese veins, brie rind). The challenge is not eating your materials halfway through!", &alice_id.0),
        ("gaming", "Cheese Heist speedrun world record broken!", "The legendary runner QuickMouse just beat Cheese Heist Any% in 42:13! The new strat uses a wall clip in the kitchen level to skip the entire cat section. Absolutely insane run.", &test_user_id.0),
        ("gaming", "What are you playing this weekend?", "I'm diving back into Hollow Knight. Trying to finally beat the Pantheon of Hallownest. What's everyone else playing? Any hidden gem recommendations?", &bob_id.0),
        ("books", "Just finished 'The Mouse and the Motorcycle'", "Re-reading a childhood classic hits different as an adult. Beverly Cleary really understood the mouse perspective — the adventure, the danger, the thrill of riding a toy motorcycle. Any other mouse-lit recommendations?", &alice_id.0),
        ("books", "Building a cozy reading corner", "I've been working on my reading nook: a little cardboard box lined with shredded paper (the soft kind!), a tiny LED lamp, and a thimble of chamomile tea. Perfect for rainy day reading. Show me your reading setups!", &test_user_id.0),
    ];

    for (topic, title, content, author) in &posts {
        let slug = title.to_lowercase().replace(' ', "-")
            .chars()
            .filter(|c| c.is_ascii_alphanumeric() || *c == '-')
            .collect::<String>();
        let final_slug = format!("{}-{}", slug, &nanoid(5));
        let topic_id: (Uuid,) = sqlx::query_as("SELECT id FROM topics WHERE name = $1")
            .bind(topic)
            .fetch_one(&post_pool)
            .await?;

        sqlx::query(
            "INSERT INTO posts (creator_id, topic_id, title, slug, content)
             VALUES ($1, $2, $3, $4, $5)
             ON CONFLICT (topic_id, slug) DO NOTHING",
        )
        .bind(author)
        .bind(topic_id.0)
        .bind(title)
        .bind(&final_slug)
        .bind(content)
        .execute(&post_pool)
        .await?;
    }
    println!("Seeded {} posts", posts.len());

    // ── Post: create comments ───────────────────────────────────────────
    let comments = [
        ("Welcome to TwoMice!", "So excited this place exists! Hello everyone!", &bob_id.0),
        ("Welcome to TwoMice!", "Finally a place where I can talk about cheese without judgment.", &alice_id.0),
        ("Forum rules and guidelines", "Good rules. Keeping it simple is the way to go.", &test_user_id.0),
        ("Best cheddar I've ever nibbled", "Was it the clothbound kind? Those are always the best.", &alice_id.0),
        ("Best cheddar I've ever nibbled", "You have to tell us which farmer's market! I need this cheese.", &test_user_id.0),
        ("Best cheddar I've ever nibbled", "Cheddar with crystals is the peak of cheese evolution.", &bob_id.0),
        ("Gouda vs Edam: the ultimate showdown", "Aged Gouda all the way. Those crunchy crystals are unbeatable.", &bob_id.0),
        ("Gouda vs Edam: the ultimate showdown", "Hot take: smoked Gouda on a cracker with a tiny bit of honey.", &alice_id.0),
        ("Building my first mechanical keyboard", "Box Jades are a bold choice for a first build! I respect it.", &test_user_id.0),
        ("Building my first mechanical keyboard", "Please post the sound test when you're done! I love clicky switches.", &bob_id.0),
        ("Rust tip: using Result with axum", "Great pattern. I do something similar but with thiserror for the error enum.", &alice_id.0),
        ("My latest watercolor: sunset over the wheat field", "The way you captured the golden light is beautiful!", &test_user_id.0),
        ("Drawn with cheese: edible art thread", "I tried this and ate my entire palette within 10 minutes.", &bob_id.0),
        ("Cheese Heist speedrun world record broken!", "The kitchen clip is going to get patched for sure. Enjoy it while it lasts!", &alice_id.0),
        ("Just finished 'The Mouse and the Motorcycle'", "Try Redwall next! Epic fantasy with mice, badgers, and the best feast descriptions ever.", &bob_id.0),
    ];

    for (post_title, content, author) in &comments {
        let post_id: (Uuid,) = sqlx::query_as(
            "SELECT id FROM posts WHERE title = $1 LIMIT 1",
        )
        .bind(post_title)
        .fetch_one(&post_pool)
        .await?;

        let hash = nanoid(5);
        sqlx::query(
            "INSERT INTO comments (hash, sender_id, post_id, content)
             VALUES ($1, $2, $3, $4)
             ON CONFLICT (post_id, hash) DO NOTHING",
        )
        .bind(&hash)
        .bind(author)
        .bind(post_id.0)
        .bind(content)
        .execute(&post_pool)
        .await?;
    }
    println!("Seeded {} comments", comments.len());

    println!("\nSeed complete! You can now login with:");
    println!("  username: mouse");
    println!("  password: {password}");

    Ok(())
}

fn nanoid(len: usize) -> String {
    use rand::Rng;
    const CHARS: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
    let mut rng = rand::thread_rng();
    (0..len)
        .map(|_| CHARS[rng.gen_range(0..CHARS.len())] as char)
        .collect()
}
