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
use tracing::{debug, error, instrument, trace};
use uuid::Uuid;

use config::Config;
use game::{FileUpdateEvent, InstallType, WatchAction, file_watcher, save_file_for_profile};
use request::{Requester, ServerError};
use saves::read_save_packed;

mod config;
mod game;
mod request;
mod saves;

const WEB_ADDRESS: &str = dotenvy_macro::dotenv!("WEB_ADDRESS");
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
        .resizable(cfg!(not(debug_assertions)))
        .theme(|_| Theme::Nord)
        .run_with(|| (State::new(), Task::done(Message::Auth { force: false })))
}

#[instrument(skip(state))]
fn update(state: &mut State, message: Message) -> Task<Message> {
    let none = Task::none();

    match message {
        Message::Auth { force } => {
            let Some(config) = &mut state.config else {
                error!("config not loaded, skipping auth");
                return none;
            };

            if !force && config.auth_key().is_some() {
                trace!("already registered, skipping auth");
            }

            if state.server_ok
                && let Ok(res) = state.client.auth()
            {
                trace!("saving auth");
                config.set_auth_key(res.key);
                let _ = config
                    .save_on_disk()
                    .inspect_err(|e| error!("failed to save config on disk: {e}"));
            }
        }
        Message::RegisterOnServer => {
            debug_assert!(
                state.server_ok,
                "should not be called if server not available"
            );

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

            let resp = match state.client.send_register(key, save_packed) {
                Ok(resp) => resp,
                Err(ServerError::WrongAuth) => {
                    error!("got unauthorized, config probably broken, suggesting to reset");
                    return Task::done(Message::ConfigProbablyBroken);
                }
                Err(_) => return none,
            };

            state
                .send_file_watches
                .send(WatchAction::watch(selected_profile))
                .unwrap();

            config.add_profile(resp.id, selected_profile);
            let _ = config
                .save_on_disk()
                .inspect_err(|e| error!("failed to save config on disk: {e}"));
            state.selected_profile.take();
        }
        Message::FileUpdated(FileUpdateEvent::SaveUpdate { name, path }) => {
            debug_assert!(
                state.server_ok,
                "should not be called if server not available"
            );

            debug!("updating file \"{name}\"");

            let Some(config) = &mut state.config else {
                error!("config not loaded, skipping saving");
                return none;
            };

            let Some(save_packed) = read_save_packed(&path) else {
                return none;
            };

            let Some(id) = config.find_profile(&name) else {
                debug!("ignoring file update for non-tracked profile \"{name}\"");
                return none;
            };
            let Some(key) = config.auth_key() else {
                error!("not authorized, skipping register update");
                return none;
            };

            if let Err(ServerError::WrongAuth) =
                state.client.send_register_update(id, key, save_packed)
            {
                error!("got unauthorized, config probably broken, suggesting to reset");
                return Task::done(Message::ConfigProbablyBroken);
            };
        }
        Message::FileUpdated(FileUpdateEvent::SaveCreate { name }) => {
            let Some(profiles) = &mut state.profiles else {
                error!("no profiles in state, skipping adding new");
                return none;
            };

            profiles.push(name);
        }
        Message::FileUpdated(FileUpdateEvent::SaveDelete { name }) => {
            let Some(profiles) = &mut state.profiles else {
                error!("no profiles in state, skipping remove");
                return none;
            };

            profiles.retain(|e| e != &name);

            let Some(config) = &mut state.config else {
                error!("config not loaded, skipping unwatch after save remove");
                return none;
            };

            // unwatch and forget
            if let Some(id) = config.find_profile(&name) {
                state
                    .send_file_watches
                    .send(WatchAction::unwatch(&name))
                    .unwrap();
                config.remove_profile(id);
                let _ = config.save_on_disk();
            }
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
            let Some(config) = &mut state.config else {
                error!("config not loaded, skipping sharing");
                return none;
            };
            let address = config.web_address().unwrap_or(WEB_ADDRESS);
            let url = format!("{address}#profile={id}");

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

            config.remove_profile(id);
            let _ = config
                .save_on_disk()
                .inspect_err(|e| error!("failed to save config on disk: {e}"));
        }
        Message::ConfigProbablyBroken => state.need_reset_config = true,
        Message::ResetConfig => {
            let Some(config) = &mut state.config else {
                error!("config not loaded, skipping forgetting");
                return none;
            };

            for profile in config.profiles() {
                state
                    .send_file_watches
                    .send(WatchAction::unwatch(&profile.name))
                    .unwrap();
            }

            config.reset_config();
            let _ = config
                .save_on_disk()
                .inspect_err(|e| error!("failed to save config on disk: {e}"));

            state.need_reset_config = false;

            return Task::done(Message::Auth { force: true });
        }
    }

    none
}

#[instrument(skip(state))]
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

    let error_msg = |s: &'static str| {
        text(s)
            .font(Font {
                style: font::Style::Italic,
                ..Default::default()
            })
            .style(|theme: &Theme| widget::text::Style {
                color: Some(theme.palette().danger),
            })
            .size(20)
            .into()
    };

    let server_ok_block: Element<_> = if state.server_ok {
        Space::new(0, 0).into()
    } else {
        error_msg("Server unavailable")
    };

    let config_reset_block: Element<_> = if state.need_reset_config {
        error_msg("Something broken, try to reset config")
    } else {
        Space::new(0, 0).into()
    };
    let config_reset_button: Element<_> = if state.need_reset_config {
        button("Reset config").on_press(Message::ResetConfig).into()
    } else {
        Space::new(0, 0).into()
    };

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
            server_ok_block,
            config_reset_block,
            row![
                button("Register").on_press_maybe(
                    if state.server_ok
                        && !state.need_reset_config
                        && let Some(ref p) = state.selected_profile
                        && config.find_profile(p).is_none()
                    {
                        Some(Message::RegisterOnServer)
                    } else {
                        None
                    }
                ),
                config_reset_button,
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

#[instrument(skip(state))]
fn subscription(state: &State) -> Subscription<Message> {
    if !state.server_ok {
        return Subscription::none();
    }
    let Some((_, ref dir)) = state.install else {
        error!("install dir is not set, skipping subscription");
        return Subscription::none();
    };
    Subscription::run_with_id(
        TypeId::of::<FileUpdateEvent>(),
        file_watcher(
            dir.to_owned(),
            #[cfg(target_os = "linux")]
            state.send_file_watches.clone(),
            Arc::clone(&state.file_watches_receiver),
        ),
    )
    .map(Message::FileUpdated)
}

#[derive(Debug, Clone)]
enum Message {
    Auth { force: bool },
    RegisterOnServer,
    SelectProfile(String),
    FileUpdated(FileUpdateEvent),
    ShareProfile(Uuid),
    ShowProfileShared,
    HideProfileShared,
    ForgetRegister(Uuid),

    ConfigProbablyBroken,
    ResetConfig,
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

    /// If server behaves good
    server_ok: bool,

    /// Client for sending requests to server
    client: Requester,

    /// App's config
    config: Option<Config>,

    /// If something broke and config reset should help
    ///
    /// Examples:
    ///
    /// - error occured when loading an app
    /// - "unauthorized" reply from server
    need_reset_config: bool,

    error: Option<Error>,
}

impl State {
    #[instrument(name = "State::new")]
    fn new() -> Self {
        let install_dir = match game::detect_install() {
            Ok(dir) => dir,
            Err(e) => {
                return Self::error(Error::GameFind(e));
            }
        };
        let profiles = match game::find_profiles(&install_dir.1) {
            Ok(profiles) => profiles,
            Err(e) => {
                return Self::error(Error::ProfilesFind(e));
            }
        };
        let mut config = match Config::new() {
            Ok(c) => c,
            Err(e) => {
                return Self::error(Error::Config(e));
            }
        };

        // remember before loading server config
        let saved_server_address = config.server_address_owned();

        let mut need_save_config = false;

        // check all known web addresses and try to load server config
        let server_config = [
            // built-in address
            Some(WEB_ADDRESS.to_string()),
            // old address from config
            config.web_address_owned(),
        ]
        .into_iter()
        .flatten()
        .map(|address| {
            debug!("checking web address {address}");
            request::get_server_config(&address)
                .inspect(|_| debug!("using web address {address}"))
                .ok()
        })
        .find_map(|config| config);
        // todo: probably handle somehow if no web is available?
        if let Some(server_config) = server_config {
            config.set_addresses(server_config);
            need_save_config = true;
        }

        let mut client = Requester::new();

        // search working server address among all known
        let server_address = [
            // built-in address
            Some(client.address().to_string()),
            // old address from config
            saved_server_address,
            // loaded address
            config.server_address_owned(),
        ]
        .into_iter()
        .flatten()
        .find(|a| {
            debug!("checking server {a}");
            client
                .with_address(a)
                .inspect_err(|e| error!("invalid address: {e}"))
                .map_err(|_| ())
                .and_then(|client| client.ping())
                .is_ok()
        });
        let server_ok = server_address.is_some();
        if let Some(server_address) = server_address {
            client
                .set_address(&server_address)
                .expect("should be checked when pinged");
        }

        let (tx, rx) = mpsc::channel();
        if server_ok {
            tx.send(WatchAction::WatchNewSaves).unwrap();
            for profile in config.profiles() {
                tx.send(WatchAction::watch(&profile.name)).unwrap();
            }
        }

        if need_save_config {
            let _ = config
                .save_on_disk()
                .inspect_err(|e| error!("failed to save config on disk: {e}"));
        }

        Self {
            install: Some(install_dir),
            profiles: Some(profiles),
            selected_profile: None,
            send_file_watches: tx,
            file_watches_receiver: Arc::new(Mutex::new(rx)),
            copied_toast_hide: None,
            server_ok,
            client,
            config: Some(config),
            need_reset_config: false,
            error: None,
        }
    }
    fn error(error: Error) -> Self {
        Self {
            error: Some(error),
            need_reset_config: true,
            ..Default::default()
        }
    }
}

impl Default for State {
    fn default() -> Self {
        let (tx, rx) = mpsc::channel();
        Self {
            install: None,
            profiles: None,
            selected_profile: None,
            send_file_watches: tx,
            file_watches_receiver: Arc::new(Mutex::new(rx)),
            copied_toast_hide: None,
            server_ok: false,
            client: Requester::new(),
            config: None,
            need_reset_config: false,
            error: None,
        }
    }
}

#[derive(Debug)]
enum Error {
    GameFind(game::DetectError),
    ProfilesFind(game::FindProfilesError),
    Config(config::ConfigError),
}
