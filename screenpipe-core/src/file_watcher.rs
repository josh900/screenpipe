use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;
use std::sync::mpsc::channel;
use std::time::Duration;

pub struct FileWatcher {
    watcher: RecommendedWatcher,
    rx: std::sync::mpsc::Receiver<notify::Result<notify::Event>>,
}

impl FileWatcher {
    pub fn new() -> notify::Result<Self> {
        let (tx, rx) = channel();
        let watcher = RecommendedWatcher::new(
            move |res| {
                tx.send(res).unwrap();
            },
            Config::default(),
        )?;
        Ok(FileWatcher { watcher, rx })
    }

    pub fn watch<P: AsRef<Path>>(&mut self, path: P) -> notify::Result<()> {
        self.watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;
        Ok(())
    }

    pub fn poll_events(&self, timeout: Duration) -> Vec<notify::Result<notify::Event>> {
        let mut events = Vec::new();
        while let Ok(event) = self.rx.recv_timeout(timeout) {
            events.push(event);
        }
        events
    }
}