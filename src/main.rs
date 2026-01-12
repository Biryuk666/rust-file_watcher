use std::env;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    if let Err(e) = run(&args) {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}

fn run(args: &[String]) -> Result<(), String> {
    if args.len() < 2 {
        return Err("Usage: file_watcher <path>".into());
    }

    let path_str: &str = &args[1];
    let path = Path::new(path_str);

    if !path.exists() {
        return Err(format!("Path does not exist: {path_str}"));
    }

    if path.is_file() {
        println!("It is a FILE: {path_str}");
    } else if path.is_dir() {
        println!("It is a DIRECTORY: {path_str}");
    } else {
        println!("It exists, but is neither a file nor a directory: {path_str}");
    }

    Ok(())
}
