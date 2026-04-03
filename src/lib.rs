mod install;
mod uninstall;
mod database;
mod ui;
mod config;

use std::path::PathBuf;
use anyhow::Result;

#[derive(Debug, Clone, Copy)]
pub enum AppMode {
    Install,
    Uninstall,
    Main,
}

pub struct AppContext {
    pub mode: AppMode,
    pub install_path: Option<PathBuf>,
}

fn main() -> Result<()> {
    env_logger::init();

    let args: Vec<String> = std::env::args().collect();
    let mode = if args.len() > 1 {
        match args[1].as_str() {
            "--uninstall" => AppMode::Uninstall,
            "--install" => AppMode::Install,
            _ => AppMode::Main,
        }
    } else {
        if config::is_installed()? {
            AppMode::Main
        } else {
            AppMode::Install
        }
    };

    match mode {
        AppMode::Install => {
            install::run_install_wizard()?;
        }
        AppMode::Uninstall => {
            uninstall::run_uninstall_wizard()?;
        }
        AppMode::Main => {
            ui::run_main_application()?;
        }
    }

    Ok(())
}