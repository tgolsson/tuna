use std::{
    collections::HashMap,
    path::PathBuf,
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc::{channel, RecvTimeoutError},
        Arc,
    },
    thread::JoinHandle,
    time::Duration,
};

use notify::{watcher, RecursiveMode, Watcher};
use toml::Value;
use tuna::{Boolean, Float32, Float64, Int32, Int64};

pub struct FileWatcher {
    shutdown: Arc<AtomicBool>,
    thread_handle: Option<JoinHandle<()>>,
    _watcher: notify::RecommendedWatcher,
}

impl Drop for FileWatcher {
    fn drop(&mut self) {
        self.shutdown.store(true, Ordering::Relaxed);

        if let Some(j) = self.thread_handle.take() {
            match j.join() {
                Ok(_) => log::debug!("File watcher stopped"),
                Err(e) => log::error!("Failed joining file watcher: {:?}", e),
            }
        }
    }
}

type TomlContents = HashMap<String, HashMap<String, Value>>;

fn apply_state(state: TomlContents) {
    for (category, kvs) in state {
        for (name, value) in kvs {
            let success = match value {
                Value::String(v) => {
                    log::warn!(
                        "unsupported string value: `{}/{} = {}`",
                        &category,
                        &name,
                        v,
                    );
                    false
                }
                Value::Datetime(v) => {
                    log::warn!(
                        "unsupported string value: `{}/{} = {}`",
                        &category,
                        &name,
                        v,
                    );
                    false
                }
                Value::Integer(v) => {
                    tuna::set::<Int64>(&category, &name, v)
                        || tuna::set::<Int32>(&category, &name, v as i32)
                }
                Value::Float(v) => {
                    tuna::set::<Float64>(&category, &name, v)
                        || tuna::set::<Float32>(&category, &name, v as f32)
                }
                Value::Boolean(v) => tuna::set::<Boolean>(&category, &name, v),

                Value::Array(_) => false,
                Value::Table(_) => false,
            };

            if !success {
                log::error!("unknown tuneable: `{}/{}`", category, name);
            }
        }
    }
}

pub fn open(path: PathBuf, period: Duration) -> anyhow::Result<FileWatcher> {
    let initial_contents = std::fs::read_to_string(&path)?;
    let initial_state = toml::from_str(&initial_contents)?;
    apply_state(initial_state);

    let should_exit = Arc::new(AtomicBool::new(false));
    let shutdown = should_exit.clone();

    let (tx, rx) = channel();
    let mut watcher = watcher(tx, period).unwrap();
    watcher
        .watch(path.clone(), RecursiveMode::Recursive)
        .unwrap();

    let thread_handle = std::thread::spawn(move || loop {
        match rx.recv_timeout(Duration::from_millis(10)) {
            Ok(_event) => {
                let contents = match std::fs::read_to_string(&path) {
                    Ok(v) => v,
                    Err(e) => {
                        log::error!("failed reading file: {}", e);
                        continue;
                    }
                };
                let state: TomlContents = match toml::from_str(&contents) {
                    Ok(v) => v,
                    Err(e) => {
                        log::error!("failed parsing file: {}", e);
                        continue;
                    }
                };

                apply_state(state);
            }
            Err(e) if e != RecvTimeoutError::Timeout => println!("watch error: {:?}", e),
            _ => {}
        }

        if should_exit.load(Ordering::Relaxed) {
            break;
        }
    });

    Ok(FileWatcher {
        shutdown,
        thread_handle: Some(thread_handle),
        _watcher: watcher,
    })
}
