use std::{
    env::home_dir,
    ffi::OsStr,
    fmt::Display,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

use iced::{
    futures::{SinkExt, Stream, StreamExt, channel::mpsc},
    stream,
};
use notify::{Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher, event::RemoveKind};
use tracing::{debug, error, trace, warn};

use crate::log::LogError;

#[cfg(target_os = "linux")]
const LINUX_STEAM_GAME_DIR: &str =
    ".local/share/Steam/steamapps/compatdata/753640/pfx/drive_c/users/steamuser";
const SAVE_DIR: &str = "AppData/LocalLow/Mobius Digital/Outer Wilds/SteamSaves";

#[derive(Debug, Clone, Copy)]
pub enum InstallType {
    Steam,
    #[expect(unused)]
    EpicGames,
}

impl Display for InstallType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            InstallType::Steam => "Steam",
            InstallType::EpicGames => "Epic Games",
        };
        s.fmt(f)
    }
}

pub fn detect_install() -> Result<(InstallType, PathBuf), DetectError> {
    let Some(home) = home_dir() else {
        return Err(DetectError::NoHome);
    };

    let mut search = vec![];

    #[cfg(target_os = "windows")]
    {
        search.push((InstallType::Steam, home.join(SAVE_DIR)));
    }
    #[cfg(target_os = "linux")]
    {
        search.push((
            InstallType::Steam,
            home.join(LINUX_STEAM_GAME_DIR).join(SAVE_DIR),
        ));
    }
    #[cfg(not(any(target_os = "windows", target_os = "linux")))]
    {
        compile_error!("unsupported os")
    }

    for (ty, path) in search {
        debug!("searching install in {}", path.display());
        if path.exists() {
            debug!("install found");
            return Ok((ty, path));
        }
    }

    error!("install not found");
    Err(DetectError::NotFound)
}

/// Find profiles names
pub fn find_profiles(path: &Path) -> Result<Vec<String>, FindProfilesError> {
    let entries = std::fs::read_dir(path)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, _>>()?;
    let entries = entries
        .iter()
        .filter(|p| p.extension().is_some_and(|ext| ext == "owprofile"))
        .filter(|p| {
            p.file_name().is_some_and(|name| path.join(name).exists())
                && p.file_stem()
                    .is_some_and(|stem| save_file_for_profile(path, stem).exists())
        })
        .map(|p| p.file_stem().expect("checked above"))
        .map(|name| name.to_str().expect("save name can be utf only"))
        .map(ToOwned::to_owned)
        .collect::<Vec<_>>();

    Ok(entries)
}

pub fn save_file_for_profile(path: &Path, name: &OsStr) -> PathBuf {
    path.join(name).join("data.owsave")
}

pub fn file_watcher(
    install_dir: PathBuf,
    watch_actions_sender: std::sync::mpsc::Sender<WatchAction>,
    watch_actions_receiver: Arc<Mutex<std::sync::mpsc::Receiver<WatchAction>>>,
) -> impl Stream<Item = FileUpdateEvent> {
    let (mut tx, mut rx) = mpsc::channel::<notify::Result<Event>>(100);

    stream::channel(100, async move |mut output| {
        let watcher = RecommendedWatcher::new(
            move |res| {
                iced::futures::executor::block_on(async {
                    tx.send(res).await.unwrap();
                })
            },
            notify::Config::default(),
        );

        let Ok(mut watcher) = watcher.log_msg("failed to start saves watcher") else {
            return;
        };

        std::thread::spawn(move || {
            for action in watch_actions_receiver.lock().unwrap().iter() {
                trace!("got watcher action: {action:?}");
                let _ = match action.kind {
                    WatchActionKind::Watch => watcher.watch(
                        &save_file_for_profile(&install_dir, OsStr::new(&action.name)),
                        RecursiveMode::NonRecursive,
                    ),
                    WatchActionKind::Unwatch => watcher.watch(
                        &save_file_for_profile(&install_dir, OsStr::new(&action.name)),
                        RecursiveMode::NonRecursive,
                    ),
                }
                .log_msg("failed to un/watch file");
            }
        });

        // used to debounce events
        //
        // todo: probably this should be HashMap<String, Instant>, but not
        // sure if it's worth to catch rare case with simultaneous updates to
        // different saves
        let mut last_name = "".to_string();
        let mut time_since_send = Instant::now();
        while let Some(res) = rx.next().await {
            let res = match res {
                Ok(res) => res,
                Err(e) => {
                    error!("failed to get event from file watcher: {e}");
                    continue;
                }
            };
            // todo: check how game writes save file
            //
            // - linux: remove + create
            // - windows: ?
            trace!(
                "notified about event: kind = {:?}, paths len = {}",
                res.kind,
                res.paths.len()
            );
            if !matches!(res.kind, EventKind::Remove(RemoveKind::File)) {
                continue;
            }

            if !res.paths.is_empty() {
                if res.paths.len() != 1 {
                    warn!("got event with number of paths > 1");
                }

                let path = res.paths[0].clone();
                let name = path
                    .parent()
                    .expect("save path should have dir")
                    .file_name()
                    .expect("save path dir should be non-empty")
                    .to_str()
                    .expect("save dir name should be valid utf")
                    .to_string();
                debug!("got event for \"{name}\"");

                if matches!(res.kind, EventKind::Remove(RemoveKind::File)) {
                    // let name = name.clone();
                    // let path = path.clone();
                    // let watch_actions_sender = watch_actions_sender.clone();
                    std::thread::scope(|s| {
                        s.spawn(|| {
                            std::thread::sleep(Duration::from_secs(1));
                            if !path.exists() {
                                return;
                            }
                            trace!("watched file edited with remove, readding");
                            watch_actions_sender
                                .send(WatchAction::watch(&name))
                                .unwrap();
                        });
                    });
                }

                if last_name == name && time_since_send.elapsed() < Duration::from_secs(1) {
                    trace!("debounced duplicate event for \"{name}\"");
                    continue;
                }

                last_name = name.clone();
                output
                    .send(FileUpdateEvent::Update { name, path })
                    .await
                    .log_msg("failed to send file update event")
                    .ok();
                time_since_send = Instant::now();

                trace!("sent event");
            } else {
                warn!("got event with empty paths");
            }
        }
    })
}

#[derive(Debug)]
pub struct WatchAction {
    kind: WatchActionKind,
    name: String,
}

#[derive(Debug)]
enum WatchActionKind {
    Watch,
    Unwatch,
}

impl WatchAction {
    pub fn watch(name: &str) -> Self {
        Self {
            kind: WatchActionKind::Watch,
            name: name.to_string(),
        }
    }
    pub fn unwatch(name: &str) -> Self {
        Self {
            kind: WatchActionKind::Unwatch,
            name: name.to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum FileUpdateEvent {
    Update { name: String, path: PathBuf },
}

#[derive(Debug, thiserror::Error)]
pub enum DetectError {
    #[error("home folder not found")]
    NoHome,
    #[error("game folder not found")]
    NotFound,
}

#[derive(Debug, thiserror::Error)]
pub enum FindProfilesError {
    #[error("{0}")]
    Io(#[from] std::io::Error),
}
