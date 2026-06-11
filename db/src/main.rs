mod seed;

use clap::{Parser, Subcommand};
use sqlx::postgres::PgPoolOptions;
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
        Commands::Seed => seed::seed().await?,
        Commands::Run { service } => migrate("run", &service)?,
        Commands::Revert { service } => migrate("revert", &service)?,
        Commands::Reset { service } => {
            reset(&service).await?;
            seed::seed().await?;
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
