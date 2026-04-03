use anyhow::Result;
use std::path::{Path, PathBuf};
use std::fs;

#[cfg(target_os = "windows")]
pub fn get_install_path_auto() -> Result<PathBuf> {
    use std::env;
    let appdata = env::var("APPDATA")?;
    Ok(PathBuf::from(appdata).join("my-library"))
}

#[cfg(target_os = "linux")]
pub fn get_install_path_auto() -> Result<PathBuf> {
    use dirs::data_dir;
    match data_dir() {
        Some(path) => Ok(path.join("my-library")),
        None => Err(anyhow::anyhow!("Could not determine data directory")),
    }
}

#[cfg(not(any(target_os = "windows", target_os = "linux")))]
pub fn get_install_path_auto() -> Result<PathBuf> {
    use dirs::home_dir;
    match home_dir() {
        Some(path) => Ok(path.join(".config").join("my-library")),
        None => Err(anyhow::anyhow!("Could not determine home directory")),
    }
}

pub fn is_installed() -> Result<bool> {
    let install_path = get_install_path_auto()?;
    let env_file = install_path.join(".env");
    Ok(env_file.exists())
}

pub fn get_database_path(install_path: &Path) -> PathBuf {
    install_path.join("main.db")
}

pub fn get_env_file_path(install_path: &Path) -> PathBuf {
    install_path.join(".env")
}

pub fn get_binary_path(install_path: &Path) -> PathBuf {
    #[cfg(target_os = "windows")]
    {
        install_path.join("my-library.exe")
    }
    #[cfg(not(target_os = "windows"))]
    {
        install_path.join("my-library")
    }
}

pub fn create_install_directory(path: &Path) -> Result<()> {
    fs::create_dir_all(path)?;
    Ok(())
}

pub fn create_empty_env_file(path: &Path) -> Result<()> {
    fs::write(path, "")?;
    Ok(())
}

#[cfg(target_os = "windows")]
pub fn get_desktop_path() -> Result<PathBuf> {
    use std::env;
    let userprofile = env::var("USERPROFILE")?;
    Ok(PathBuf::from(userprofile).join("Desktop"))
}

#[cfg(target_os = "linux")]
pub fn get_desktop_path() -> Result<PathBuf> {
    use dirs::desktop_dir;
    match desktop_dir() {
        Some(path) => Ok(path),
        None => Err(anyhow::anyhow!("Could not determine desktop directory")),
    }
}