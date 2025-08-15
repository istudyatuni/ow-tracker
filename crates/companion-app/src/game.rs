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
#[cfg(target_os = "windows")]
use notify::event::ModifyKind;
#[cfg(target_os = "linux")]
use notify::event::{CreateKind, RemoveKind};
use notify::{Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use tracing::{debug, error, instrument, trace, warn};

#[cfg(target_os = "linux")]
const LINUX_STEAM_GAME_DIR: &str =
    ".local/share/Steam/steamapps/compatdata/753640/pfx/drive_c/users/steamuser";
const SAVE_DIR: &str = "AppData/LocalLow/Mobius Digital/Outer Wilds/SteamSaves";

/// Duration during which events must be debounced
const DEBOUNCE_DUR: Duration = Duration::from_secs(1);

/// How long to wait before check if file was created again when detecting rename
#[cfg(target_os = "linux")]
const REMOVE_CREATE_WAIT: Duration = Duration::from_secs(1);

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

#[instrument]
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
        if path.exists() {
            trace!("found game path: {}", path.display());
            return Ok((ty, path));
        }
    }

    error!("install not found");
    Err(DetectError::NotFound)
}

/// Find profiles names
#[instrument]
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

#[instrument(skip_all, fields(path = install_dir.display().to_string()))]
pub fn file_watcher(
    install_dir: PathBuf,
    #[rustfmt::skip]
    #[cfg(target_os = "linux")]
    watch_actions_sender: std::sync::mpsc::Sender<WatchAction>,
    watch_actions_receiver: Arc<Mutex<std::sync::mpsc::Receiver<WatchAction>>>,
) -> impl Stream<Item = FileUpdateEvent> {
    // using async channel here because otherwise channel won't be dropped on resubscription
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

        let Ok(mut watcher) = watcher.inspect_err(|e| error!("failed to start saves watcher: {e}"))
        else {
            return;
        };

        std::thread::spawn(move || {
            for action in watch_actions_receiver.lock().unwrap().iter() {
                trace!("got watcher action: {action:?}");
                let _ = match action {
                    WatchAction::WatchNewSaves => {
                        watcher.watch(&install_dir, RecursiveMode::NonRecursive)
                    }
                    WatchAction::WatchProfile { name } => watcher.watch(
                        &save_file_for_profile(&install_dir, OsStr::new(&name)),
                        RecursiveMode::NonRecursive,
                    ),
                    WatchAction::UnwatchProfile { name } => {
                        watcher.unwatch(&save_file_for_profile(&install_dir, OsStr::new(&name)))
                    }
                }
                .inspect_err(|e| error!("failed to un/watch file: {e}"));
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

            if !matches!(res.kind, EventKind::Access(_)) {
                trace!(
                    "notified about event: kind = {:?}, paths = {:?}",
                    res.kind, res.paths,
                );
            }

            // saving in separate variable because can't check res.paths[0].is_dir() after folder was deleted
            let is_folder_event = matches!(
                res.kind,
                EventKind::Create(CreateKind::Folder) | EventKind::Remove(RemoveKind::Folder)
            );

            #[cfg(target_os = "linux")]
            if !matches!(
                res.kind,
                // save update
                EventKind::Remove(RemoveKind::File)
                    // new save created
                    | EventKind::Create(CreateKind::Folder)
                    // save removed
                    | EventKind::Remove(RemoveKind::Folder)
            ) {
                continue;
            }
            #[cfg(target_os = "windows")]
            if !matches!(
                res.kind,
                EventKind::Modify(ModifyKind::Any)
                    // todo: check
                    | EventKind::Create(CreateKind::Folder)
                    | EventKind::Remove(RemoveKind::Folder)
            ) {
                continue;
            }

            let [path] = res.paths.as_slice() else {
                continue;
            };

            if path.file_name().is_some_and(|name| name == "data.owsave") {
                // save file updated
                let name = path
                    .parent()
                    .expect("save path should have dir")
                    .file_name()
                    .expect("save path dir should be non-empty")
                    .to_str()
                    .expect("save dir name should be valid utf")
                    .to_string();
                debug!("got update event for \"{name}\"");

                // on linux watcher stops tracking deleted file, but on windows does not
                #[cfg(target_os = "linux")]
                if matches!(res.kind, EventKind::Remove(RemoveKind::File)) {
                    std::thread::scope(|s| {
                        s.spawn(|| {
                            std::thread::sleep(REMOVE_CREATE_WAIT);
                            if !path.exists() {
                                return;
                            }
                            trace!("[linux] watched file edited with remove, readding");
                            watch_actions_sender
                                .send(WatchAction::watch(&name))
                                .unwrap();
                        });
                    });
                }

                if last_name == name && time_since_send.elapsed() < DEBOUNCE_DUR {
                    trace!("debounced duplicate event for \"{name}\"");
                    continue;
                }

                last_name = name.clone();
                output
                    .send(FileUpdateEvent::SaveUpdate {
                        name,
                        path: path.clone(),
                    })
                    .await
                    .inspect_err(|e| error!("failed to send file update event: {e}"))
                    .ok();
                time_since_send = Instant::now();

                trace!("sent file update event");
            } else if is_folder_event {
                // save created/deleted
                let path = res.paths[0].clone();
                let name = path
                    .file_name()
                    .expect("save path dir should be non-empty")
                    .to_str()
                    .expect("save dir name should be valid utf")
                    .to_string();
                debug!("got create/delete event for \"{name}\"");

                let event = match res.kind {
                    EventKind::Create(_) => FileUpdateEvent::SaveCreate { name },
                    EventKind::Remove(_) => FileUpdateEvent::SaveDelete { name },
                    kind => {
                        error!("got unknown event kind {kind:?} for save create/delete");
                        continue;
                    }
                };

                output
                    .send(event)
                    .await
                    .inspect_err(|e| error!("failed to send save create/delete event: {e}"))
                    .ok();
            }
        }
    })
}

#[derive(Debug)]
pub enum WatchAction {
    WatchNewSaves,
    WatchProfile { name: String },
    UnwatchProfile { name: String },
}

impl WatchAction {
    pub fn watch(name: &str) -> Self {
        Self::WatchProfile {
            name: name.to_string(),
        }
    }
    pub fn unwatch(name: &str) -> Self {
        Self::UnwatchProfile {
            name: name.to_string(),
        }
    }
}

#[derive(Debug, Clone)]
#[expect(clippy::enum_variant_names)]
pub enum FileUpdateEvent {
    SaveCreate { name: String },
    SaveUpdate { name: String, path: PathBuf },
    SaveDelete { name: String },
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
