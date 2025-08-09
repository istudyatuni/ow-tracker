#![cfg_attr(
    all(
        not(debug_assertions),
        target_os = "windows",
        not(feature = "windows_console")
    ),
    windows_subsystem = "windows"
)]

use std::any::TypeId;
use std::ffi::OsStr;
use std::path::PathBuf;
use std::sync::{Arc, LazyLock, Mutex, mpsc};
use std::time::Duration;

use iced::task::Handle;
use iced::widget::{self, Column, Space, button, column, container, row, text};
use iced::{Element, Fill, Font, Subscription, Task, Theme, clipboard, font};
use tracing::{debug, error, trace};
use uuid::Uuid;

use config::Config;
use game::{FileUpdateEvent, InstallType, WatchAction, file_watcher, save_file_for_profile};
use log::LogError;
use request::{send_register, send_register_update};
use saves::read_save_packed;

mod config;
mod game;
mod log;
mod request;
mod saves;

const WEB_ORIGIN: &str = dotenvy_macro::dotenv!("WEB_ORIGIN");
const SERVER_HOST: &str = dotenvy_macro::dotenv!("SERVER_HOST");
static SERVER_PORT: LazyLock<u16> = LazyLock::new(|| {
    dotenvy_macro::dotenv!("SERVER_PORT")
        .parse()
        .expect("server port should be a valid number")
});
static SERVER_ADDRESS: LazyLock<String> =
    LazyLock::new(|| format!("{SERVER_HOST}:{}", *SERVER_PORT));

const COPIED_TOAST_DURATION: Duration = Duration::from_secs(2);

pub fn main() -> iced::Result {
    common::logger::init_logging(env!("CARGO_CRATE_NAME"));

    iced::application("Outer Wilds Tracker - Companion App", update, view)
        .subscription(subscription)
        .window_size((1200.0, 800.0))
        .resizable(false)
        .theme(|_| Theme::Nord)
        .run()
}

fn update(state: &mut State, message: Message) -> Task<Message> {
    let none = Task::none();

    match message {
        Message::RegisterOnServer => {
            let selected_profile = state
                .selected_profile
                .as_ref()
                .expect("selected_profile should be defined before register");
            let install_dir = state
                .install
                .as_ref()
                .expect("install dir should be defined before register");
            let save_path = save_file_for_profile(&install_dir.1, OsStr::new(selected_profile));

            let Some(save_packed) = read_save_packed(&save_path) else {
                return none;
            };

            let Some(config) = &mut state.config else {
                error!("config not loaded, skipping saving");
                return none;
            };
            let Some(key) = config.auth_key() else {
                error!("not authorized, skipping register");
                return none;
            };

            let Ok(resp) = send_register(key, save_packed) else {
                return none;
            };

            state
                .send_file_watches
                .send(WatchAction::watch(selected_profile))
                .unwrap();

            config.add_register(resp.id, selected_profile);
            let _ = config
                .save_on_disk()
                .log_msg("failed to save config on disk");
            state.selected_profile.take();
        }
        Message::FileUpdated(FileUpdateEvent::Update { name, path }) => {
            debug!("updating file {name}");

            let Some(config) = &mut state.config else {
                error!("config not loaded, skipping saving");
                return none;
            };

            let Some(save_packed) = read_save_packed(&path) else {
                return none;
            };

            let Some(id) = config.find_profile(&name) else {
                debug!("ignoring file update for non-tracked profile");
                return none;
            };
            let Some(key) = config.auth_key() else {
                error!("not authorized, skipping register");
                return none;
            };

            let _ = send_register_update(id, key, save_packed);
        }
        Message::SelectProfile(name) => {
            if let Some(ref current) = state.selected_profile
                && current == &name
            {
                return none;
            }
            state.selected_profile.replace(name.clone());
        }
        Message::ShareProfile(id) => {
            let url = format!("{WEB_ORIGIN}/ow-tracker?profile={id}");

            return clipboard::write(url)
                .chain(Task::done(Message::HideProfileShared))
                .chain(Task::done(Message::ShowProfileShared));
        }
        Message::ShowProfileShared => {
            let (task, abort) = Task::future(async {
                std::thread::sleep(COPIED_TOAST_DURATION);
                Message::HideProfileShared
            })
            .abortable();
            state.copied_toast_hide.replace(abort);
            return task;
        }
        Message::HideProfileShared => {
            state.copied_toast_hide.take();
        }
        Message::ForgetRegister(id) => {
            let Some(config) = &mut state.config else {
                error!("config not loaded, skipping forgetting");
                return none;
            };

            state
                .send_file_watches
                .send(WatchAction::unwatch(
                    config
                        .get_profile(id)
                        .map(|p| &p.name)
                        .expect("profile should be in config when unwatch"),
                ))
                .unwrap();

            config.remove_register(id);
            let _ = config
                .save_on_disk()
                .log_msg("failed to save config on disk");
        }
    }

    none
}

fn view(state: &State) -> Element<'_, Message> {
    if let Some(ref err) = state.error {
        let inner: Element<_> = match err {
            Error::GameFind(e) => match e {
                game::DetectError::NotFound => text("Game installation not found").size(20).into(),
                _ => text("Game installation not found: {e}").size(20).into(),
            },
            Error::ProfilesFind(e) => text(format!("Failed to find profiles: {e}"))
                .size(20)
                .into(),
            Error::Config(e) => text(format!("Failed to load config: {e}")).size(20).into(),
        };
        return container(inner).center_x(Fill).center_y(Fill).into();
    }

    let install_dir = state.install.clone().unwrap();
    let config = state.config.clone().unwrap();
    let mut profiles = state.profiles.clone().unwrap();

    profiles.sort_unstable();
    let profiles = profiles.into_iter().map(|p| {
        let cloned_p = p.clone();
        let profile_register = config.profiles().iter().find(|profile| profile.name == p);
        row![
            text(format!("- {p}")).width(200).size(20),
            row![
                button("Select")
                    .style(move |theme: &Theme, _| {
                        let p = &cloned_p;
                        let palette = theme.palette();
                        if state
                            .selected_profile
                            .as_ref()
                            .is_some_and(|name| name == p)
                        {
                            widget::button::Style::default().with_background(palette.success)
                        } else {
                            widget::button::Style::default()
                                .with_background(palette.primary.scale_alpha(0.5))
                        }
                    })
                    .on_press(Message::SelectProfile(p.to_string())),
                button("Forget register")
                    .on_press_maybe(profile_register.map(|p| Message::ForgetRegister(p.id))),
                button("Share")
                    .on_press_maybe(profile_register.map(|p| Message::ShareProfile(p.id))),
            ]
            .spacing(10),
        ]
        .width(500)
        .into()
    });

    let copied_block: Element<_> = if state.copied_toast_hide.is_some() {
        text("Copied")
            .font(Font {
                style: font::Style::Italic,
                ..Default::default()
            })
            .style(|theme: &Theme| widget::text::Style {
                color: Some(theme.palette().success),
            })
            .size(20)
            .into()
    } else {
        Space::new(0, 0).into()
    };

    container(
        column![
            row![
                text("Game installation found! Type: ").size(20),
                text(install_dir.0.to_string()).size(20),
            ],
            text("Select profile and press \"Register\"").size(20),
            // todo: show something when no profiles found
            text("Found profiles:").size(20),
            Column::from_iter(profiles),
            row![
                button("Register").on_press_maybe(
                    if state
                        .selected_profile
                        .as_ref()
                        .is_some_and(|p| config.find_profile(p).is_none())
                    {
                        Some(Message::RegisterOnServer)
                    } else {
                        None
                    }
                ),
                copied_block,
            ]
            .spacing(10),
        ]
        .spacing(10),
    )
    .padding(10)
    .center_x(Fill)
    .center_y(Fill)
    .into()
}

fn subscription(state: &State) -> Subscription<Message> {
    let Some((_, ref dir)) = state.install else {
        error!("install dir is not set, skipping subscription");
        return Subscription::none();
    };
    Subscription::run_with_id(
        TypeId::of::<FileUpdateEvent>(),
        file_watcher(
            dir.to_owned(),
            state.send_file_watches.clone(),
            Arc::clone(&state.file_watches_receiver),
        ),
    )
    .map(Message::FileUpdated)
}

#[derive(Debug, Clone)]
enum Message {
    RegisterOnServer,
    SelectProfile(String),
    FileUpdated(FileUpdateEvent),
    ShareProfile(Uuid),
    ShowProfileShared,
    HideProfileShared,
    ForgetRegister(Uuid),
}

#[derive(Debug)]
struct State {
    /// Game installation
    install: Option<(InstallType, PathBuf)>,
    /// List of profiles names
    profiles: Option<Vec<String>>,
    /// Profile, selected in UI
    selected_profile: Option<String>,

    /// Send when file should be un/watched
    send_file_watches: mpsc::Sender<WatchAction>,
    /// Receiver for file watch thread
    file_watches_receiver: Arc<Mutex<mpsc::Receiver<WatchAction>>>,

    /// Handle to hide "copied" toast
    copied_toast_hide: Option<Handle>,

    /// App's config
    config: Option<Config>,

    error: Option<Error>,
}

impl State {
    fn new() -> Self {
        let install_dir = match game::detect_install() {
            Ok(dir) => dir,
            Err(e) => {
                return Self {
                    error: Some(Error::GameFind(e)),
                    ..Default::default()
                };
            }
        };
        let profiles = match game::find_profiles(&install_dir.1) {
            Ok(profiles) => profiles,
            Err(e) => {
                return Self {
                    error: Some(Error::ProfilesFind(e)),
                    ..Default::default()
                };
            }
        };
        let mut config = match Config::new() {
            Ok(c) => c,
            Err(e) => {
                return Self {
                    error: Some(Error::Config(e)),
                    ..Default::default()
                };
            }
        };

        if config.auth_key().is_none()
            && let Ok(res) = request::auth()
        {
            trace!("saving auth");
            config.set_auth_key(res.key);
            let _ = config
                .save_on_disk()
                .log_msg("failed to save config on disk");
        } else {
            trace!("already registered, skipping auth");
        };

        let (tx, rx) = mpsc::channel();
        for profile in config.profiles() {
            tx.send(WatchAction::watch(&profile.name)).unwrap();
        }
        Self {
            install: Some(install_dir),
            profiles: Some(profiles),
            selected_profile: None,
            send_file_watches: tx,
            file_watches_receiver: Arc::new(Mutex::new(rx)),
            copied_toast_hide: None,
            config: Some(config),
            error: None,
        }
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
enum Error {
    GameFind(game::DetectError),
    ProfilesFind(game::FindProfilesError),
    Config(config::ConfigError),
}
