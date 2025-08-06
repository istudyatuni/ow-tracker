#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::path::PathBuf;
use std::sync::LazyLock;

use config::Config;
use iced::widget::{Column, button, column, container, horizontal_space, row, text};
use iced::{Element, Fill, Theme};
use tracing::error;
use uuid::Uuid;

mod config;
mod game;

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

            let client = reqwest::blocking::Client::new();
            let Ok(resp) = client
                .post(
                    (*SERVER_ADDRESS)
                        .parse::<reqwest::Url>()
                        .expect("server url should be valid")
                        .join("/api/register")
                        .expect("url path should be valid"),
                )
                .json(&server::RegisterRequest { save: Vec::new() })
                .send()
                .inspect_err(|e| error!("failed to send register request: {e}"))
            else {
                return;
            };
            let Ok(resp) = resp
                .json::<server::RegisterResponse>()
                .inspect_err(|e| error!("failed to parse register response: {e}"))
            else {
                return;
            };
            state.registered_id.replace(resp.id);

            let Some(config) = &mut state.config else {
                error!("config not loaded, skipping saving");
                return;
            };

            config.add_register(resp.id, selected_profile);
            if let Err(e) = config.save_on_disk() {
                error!("failed to save config on dist: {e}");
            }
            state.selected_profile.take();
        }
        Message::SelectProfile(name) => {
            if state
                .selected_profile
                .as_ref()
                .is_some_and(|current| current == &name)
            {
                state.selected_profile.take();
            } else {
                state.selected_profile.replace(name);
            }
        }
        Message::ForgetRegister(_id) => todo!(),
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
    let profiles = profiles.iter().map(|p| {
        row![
            text(format!("- {p}")).width(200).size(20),
            row![
                button(
                    // todo: change button color, not text
                    if state
                        .selected_profile
                        .as_ref()
                        .is_some_and(|name| name == p)
                    {
                        "Unselect"
                    } else {
                        "Select"
                    }
                )
                .on_press(Message::SelectProfile(p.to_string())),
                horizontal_space().width(10),
                button("Forget register").on_press_maybe(
                    config
                        .profiles()
                        .iter()
                        .find(|profile| &profile.name == p)
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
            text("Found profiles:").size(20),
            Column::from_iter(profiles),
            button("Register").on_press_maybe(if state.selected_profile.is_some() {
                Some(Message::RegisterOnServer)
            } else {
                None
            }),
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
    selected_profile: Option<String>,
    registered_id: Option<Uuid>,

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
            registered_id: None,
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
