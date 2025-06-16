mod app;
mod config;
pub(crate) mod evaluator;
pub(crate) mod state;
mod ui;

use anyhow::Result;
use clap::Parser;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

/// PrEval - A cross-platform TUI for running and monitoring prompt evaluation tests
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Evaluator command to run
    evaluator: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing subscriber with env filter
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_target(false)
                .with_thread_ids(false)
                .with_thread_names(false),
        )
        .with(
            EnvFilter::try_from_default_env()
                .or_else(|_| EnvFilter::try_new("info"))
                .unwrap(),
        )
        .init();

    // Parse command line arguments
    let cli = Cli::parse();

    // If no evaluator specified, show help
    if cli.evaluator.is_none() {
        println!(
            "PrEval - A cross-platform TUI for running and monitoring prompt evaluation tests\n"
        );
        println!("Usage: preval <EVALUATOR>\n");
        println!("Arguments:");
        println!("  <EVALUATOR>  Evaluator command to run\n");
        println!("Options:");
        println!("  -h, --help     Print help");
        println!("  -V, --version  Print version");
        return Ok(());
    }

    // Create and run the application
    let mut app = app::App::new(cli.evaluator);
    app.run().await?;

    Ok(())
}
