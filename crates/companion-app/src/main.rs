#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::ffi::OsStr;
use std::path::PathBuf;
use std::sync::LazyLock;

use iced::widget::button::Style;
use iced::widget::{Column, button, column, container, horizontal_space, row, text};
use iced::{Element, Fill, Theme};
use tracing::error;
use uuid::Uuid;

use config::{Config, Profile};
use game::save_file_for_profile;
use log::LogError;
use saves::read_save_packed;

mod config;
mod game;
mod log;
mod saves;

const SERVER_HOST: &str = dotenvy_macro::dotenv!("SERVER_HOST");
static SERVER_PORT: LazyLock<u16> = LazyLock::new(|| {
    dotenvy_macro::dotenv!("SERVER_PORT")
        .parse()
        .expect("server port should be a valid number")
});
static SERVER_ADDRESS: LazyLock<String> =
    LazyLock::new(|| format!("{SERVER_HOST}:{}", *SERVER_PORT));

pub fn main() -> iced::Result {
    let _ = tracing::subscriber::set_global_default(
        tracing_subscriber::FmtSubscriber::builder()
            .with_target(false)
            .with_max_level(tracing::Level::DEBUG)
            .finish(),
    )
    .inspect_err(|e| eprintln!("failed to initialize logging: {e}"));

    iced::application("Outer Wilds Tracker - Companion App", update, view)
        .window_size((1200.0, 800.0))
        .resizable(false)
        .theme(|_| Theme::Nord)
        .run()
}

fn update(state: &mut State, message: Message) {
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
            let save_path = save_file_for_profile(install_dir, OsStr::new(selected_profile));

            let Some(save_packed) = read_save_packed(&save_path) else {
                return;
            };

            let client = reqwest::blocking::Client::new();
            let Ok(resp) = client
                .post(
                    (*SERVER_ADDRESS)
                        .parse::<reqwest::Url>()
                        .expect("server url should be valid")
                        .join("/api/register")
                        .expect("url path should be valid"),
                )
                .json(&common::server_models::RegisterRequest { save: save_packed })
                .send()
                .log_msg("failed to send register request")
            else {
                return;
            };
            if resp.error_for_status_ref().is_err() {
                match resp.text() {
                    Ok(text) => error!("error registering save: {text}"),
                    Err(e) => error!("error registering save (failed to get response text: {e:?})"),
                }
                return;
            }
            let Ok(resp) = resp
                .json::<common::server_models::RegisterResponse>()
                .log_msg("failed to parse register response")
            else {
                return;
            };

            let Some(config) = &mut state.config else {
                error!("config not loaded, skipping saving");
                return;
            };

            config.add_register(resp.id, selected_profile);
            let _ = config
                .save_on_disk()
                .log_msg("failed to save config on disk");
            state.selected_profile.take();
        }
        Message::SelectProfile(name) => {
            if let Some(ref current) = state.selected_profile
                && current == &name
            {
                return;
            }
            state.selected_profile.replace(name.clone());
        }
        Message::ForgetRegister(id) => {
            let Some(config) = &mut state.config else {
                error!("config not loaded, skipping forgetting");
                return;
            };

            config.remove_register(id);
            let _ = config
                .save_on_disk()
                .log_msg("failed to save config on disk");
        }
    }
}

fn view(state: &State) -> Element<Message> {
    if let Some(ref err) = state.error {
        let inner: Element<_> = match err {
            Error::GameFind(e) => match e {
                game::DetectError::NotFound(path_bufs) => {
                    let errors = path_bufs
                        .iter()
                        .map(|p| text(format!("- {}", p.display())).size(20).into());
                    column![
                        text("Game installation not found, searched at").size(20),
                        Column::from_vec(errors.collect()),
                    ]
                    .into()
                }
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
                            Style::default().with_background(palette.success)
                        } else {
                            Style::default().with_background(palette.primary.scale_alpha(0.5))
                        }
                    })
                    .on_press(Message::SelectProfile(p.to_string())),
                horizontal_space().width(10),
                button("Forget register").on_press_maybe(
                    config
                        .profiles()
                        .iter()
                        .find(|profile| profile.name == p)
                        .map(|p| Message::ForgetRegister(p.id))
                ),
            ],
        ]
        .width(500)
        .into()
    });

    container(
        column![
            text("Game installation found!").size(20),
            text(install_dir.display().to_string()).width(600).size(20),
            // todo: show something when no profiles found
            text("Found profiles:").size(20),
            Column::from_iter(profiles),
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
        ]
        .spacing(10),
    )
    .padding(10)
    .center_x(Fill)
    .center_y(Fill)
    .into()
}

#[derive(Debug, Clone)]
enum Message {
    RegisterOnServer,
    SelectProfile(String),
    ForgetRegister(Uuid),
}

#[derive(Debug)]
struct State {
    /// Game installation
    install: Option<PathBuf>,
    /// List of profiles names
    profiles: Option<Vec<String>>,
    /// Profile, selected in UI
    selected_profile: Option<String>,
    watched_profiles: Vec<Profile>,

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
        let profiles = match game::find_profiles(&install_dir) {
            Ok(profiles) => profiles,
            Err(e) => {
                return Self {
                    error: Some(Error::ProfilesFind(e)),
                    ..Default::default()
                };
            }
        };
        let config = match Config::new() {
            Ok(c) => c,
            Err(e) => {
                return Self {
                    error: Some(Error::Config(e)),
                    ..Default::default()
                };
            }
        };
        Self {
            install: Some(install_dir),
            profiles: Some(profiles),
            selected_profile: None,
            watched_profiles: vec![],
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
