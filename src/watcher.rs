use std::{
    collections::HashSet,
    path::{Path, PathBuf},
    sync::mpsc,
    time::{Duration, Instant},
};

use notify::{
    Event, EventKind, RecursiveMode, Watcher,
    event::{ModifyKind, RenameMode},
};

use crate::scanner::{self, Stats};

pub fn watch_and_scan(root: &Path) -> notify::Result<()> {
    let (sender, receiver) = mpsc::channel::<notify::Result<Event>>();
    let mut watcher = notify::recommended_watcher(sender)?;
    watcher.watch(root, RecursiveMode::Recursive)?;

    let debounce_window = Duration::from_millis(400);

    let mut pending: HashSet<PathBuf> = HashSet::new();
    let mut last_hit: Option<Instant> = None;

    let mut stats = Stats::default();

    tracing::info!(
        "Watching (debounce={}ms): {}",
        debounce_window.as_millis(),
        root.display()
    );

    loop {
        match receiver.recv_timeout(debounce_window) {
            Ok(Ok(event)) => {
                if should_process(&event.kind) {
                    for path in event.paths {
                        pending.insert(path);
                    }
                    last_hit = Some(Instant::now());
                }
            }

            Ok(Err(err)) => {
                tracing::warn!(?err, "watch error");
            }
            Err(mpsc::RecvTimeoutError::Timeout) => {
                if let Some(t) = last_hit {
                    if !pending.is_empty() && t.elapsed() >= debounce_window {
                        flush_batch(&pending, &mut stats);
                        pending.clear();
                        last_hit = None;
                    }

                    tracing::info!(
                        files = stats.files,
                        total_bytes = stats.total_bytes,
                        "stats so far"
                    );
                }
            }
            Err(mpsc::RecvTimeoutError::Disconnected) => break,
        }
    }

    Ok(())
}

fn should_process(kind: &EventKind) -> bool {
    match kind {
        EventKind::Create(_) => true,
        EventKind::Remove(_) => true,
        EventKind::Modify(mod_kind) => match mod_kind {
            ModifyKind::Data(_) => true,
            ModifyKind::Name(RenameMode::Any) => true,
            ModifyKind::Name(_) => true,
            ModifyKind::Metadata(_) => true,
            ModifyKind::Any => true,
            _ => false,
        },
        _ => false,
    }
}

fn flush_batch(paths: &HashSet<PathBuf>, stats: &mut Stats) {
    tracing::info!("--- debounce batch ({} paths) ---", paths.len());
    for path in paths {
        if !path.exists() {
            tracing::info!("gone: {}", path.display());
        }

        if let Err(e) = scanner::scan_path(path.as_path(), stats) {
            tracing::warn!("scan failed for {}: {e}", path.display());
        }
    }
}
