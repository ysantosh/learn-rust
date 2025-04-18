use anyhow::Result;
use clap::Parser;
use sqlx::postgres::PgPool;
use std::time::Instant;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    database_url: String,
}

async fn ping_database(database_url: &str) -> Result<()> {
    let mut start = Instant::now();
    let pool = PgPool::connect(database_url).await?;
    let conn_elapsed = start.elapsed().as_millis();

    start = Instant::now();
    sqlx::query("SELECT 1").execute(&pool).await?;
    let elapsed = start.elapsed().as_millis();
    println!(
        "Connect time: {} ms, Ping time: {} ms ",
        conn_elapsed, elapsed
    );

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    for i in 0..5 {
        println!("Iteration: {}", i);
        ping_database(&args.database_url).await?;
    }

    Ok(())
}
