use anyhow::Result;
use std::path::Path;
use std::fs;

pub fn perform_installation(install_path: &Path) -> Result<()> {
    log::info!("Starting installation to: {}", install_path.display());

    crate::config::create_install_directory(install_path)?;

    let env_file_path = crate::config::get_env_file_path(install_path);
    crate::config::create_empty_env_file(&env_file_path)?;

    let db_path = crate::config::get_database_path(install_path);
    crate::database::init_database(&db_path)?;

    copy_binary(install_path)?;

    log::info!("Installation completed successfully");
    Ok(())
}

fn copy_binary(install_path: &Path) -> Result<()> {
    let current_exe = std::env::current_exe()?;
    let binary_path = crate::config::get_binary_path(install_path);

    fs::copy(&current_exe, &binary_path)?;

    #[cfg(target_os = "linux")]
    {
        use std::os::unix::fs::PermissionsExt;
        let perms = fs::Permissions::from_mode(0o755);
        fs::set_permissions(&binary_path, perms)?;
    }

    log::info!("Binary copied to: {}", binary_path.display());
    Ok(())
}