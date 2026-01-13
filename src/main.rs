use std::{env, path::Path};

mod scanner;
mod watcher;

fn main() {
    tracing_subscriber::fmt::init();

    let args: Vec<String> = env::args().collect();

    if let Err(e) = run(&args) {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}

fn run(args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    if args.len() < 2 {
        return Err("Usage: file_watcher <path>".into());
    }

    let root = Path::new(&args[1]);

    if !root.exists() {
        return Err(format!("Path does not exist: {}", root.display()).into());
    }

    tracing::info!("Root: {}", root.display());
    tracing::info!("Press Ctrl+C to stop.");

    watcher::watch_and_scan(root)?;

    Ok(())
}
