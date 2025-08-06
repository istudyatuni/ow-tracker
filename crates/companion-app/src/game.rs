use std::{
    env::home_dir,
    path::{Path, PathBuf},
};

#[cfg(target_os = "linux")]
const LINUX_STEAM_GAME_DIR: &str =
    ".local/share/Steam/steamapps/compatdata/753640/pfx/drive_c/users/steamuser";
const SAVE_DIR: &str = "AppData/LocalLow/Mobius Digital/Outer Wilds/SteamSaves";

pub fn detect_install() -> Result<PathBuf, DetectError> {
    let Some(home) = home_dir() else {
        return Err(DetectError::NoHome);
    };

    let mut search = vec![];

    #[cfg(target_os = "windows")]
    {
        search.push(home.join(SAVE_DIR));
    }
    #[cfg(target_os = "linux")]
    {
        search.push(home.join(LINUX_STEAM_GAME_DIR).join(SAVE_DIR));
    }
    #[cfg(not(any(target_os = "windows", target_os = "linux")))]
    {
        compile_error!("unsupported os")
    }

    for path in &search {
        if path.exists() {
            return Ok(path.clone());
        }
    }

    Err(DetectError::NotFound(search))
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
                    .is_some_and(|stem| path.join(stem).join("data.owsave").exists())
        })
        .map(|p| p.file_stem().expect("checked above"))
        .map(|name| name.to_str().expect("save name can be utf only"))
        .map(ToOwned::to_owned)
        .collect::<Vec<_>>();

    Ok(entries)
}

#[derive(Debug, thiserror::Error)]
pub enum DetectError {
    #[error("home folder not found")]
    NoHome,
    #[error("game folder not found")]
    NotFound(Vec<PathBuf>),
}

#[derive(Debug, thiserror::Error)]
pub enum FindProfilesError {
    #[error("{0}")]
    Io(#[from] std::io::Error),
}
