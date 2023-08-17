pub mod jobs;
pub mod scrape;

use clap::{Parser, Subcommand};

#[derive(Subcommand)]
enum Commands {
    SyncNovels,
    SyncStatistics,
}

#[derive(Parser)]
#[command(author, version)]
#[command(about = "jinjiang-cli", long_about = "sync novels and statistics from jinjiang")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::SyncNovels) => {
            let _ = jobs::sync_book_list().await?;
        }
        Some(Commands::SyncStatistics) => {
            let _ = jobs::sync_novel_statistics().await?;
        }
        _ => {
            println!("nothing");
        }
    };

    Ok(())
}
