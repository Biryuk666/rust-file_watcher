use std::{
    fs,
    path::{Path, PathBuf},
};

#[derive(Default, Debug, Clone)]
pub struct Stats {
    pub files: u64,
    pub total_bytes: u64,
}

pub fn scan_path(path: &Path, stats: &mut Stats) -> std::io::Result<()> {
    if path.is_file() {
        scan_file(path, stats)?;
    } else if path.is_dir() {
        scan_dir(path, stats)?;
    }
    Ok(())
}

fn scan_file(path: &Path, stats: &mut Stats) -> std::io::Result<()> {
    let meta = fs::metadata(path)?;
    let size = meta.len();

    // Тут пока просто печать; позже перейдём на tracing везде единообразно
    println!("FILE {} ({} bytes)", path.display(), size);

    stats.files += 1;
    stats.total_bytes += size;
    Ok(())
}

fn scan_dir(dir: &Path, stats: &mut Stats) -> std::io::Result<()> {
    let entries = fs::read_dir(dir)?;

    for entry in entries {
        let entry = entry?;
        let path: PathBuf = entry.path();

        if path.is_dir() {
            scan_dir(&path, stats)?;
        } else if path.is_file() {
            scan_file(&path, stats)?;
        }
    }

    Ok(())
}
