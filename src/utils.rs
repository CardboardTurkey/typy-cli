use std::{fs, process::Command};
use dirs::home_dir;
use anyhow::{Result, Context};

use crossterm::event::{KeyCode, KeyModifiers};

pub const LINE_LENGTH: i32 = 70;

pub fn close_typy(code: &KeyCode, modifiers: &KeyModifiers) -> Option<()> {
    match code {
        KeyCode::Esc => Some(()),
        KeyCode::Char('c') if modifiers.contains(KeyModifiers::CONTROL) => Some(()),
        _ => None,
    }
}

pub fn calc_middle_for_text() -> Result<(u16, u16)> {
    let (cols, rows) = crossterm::terminal::size()
        .context("Failed to get terminal size")?;
    let x = cols / 2 - (LINE_LENGTH / 2) as u16;
    let y = rows / 2 - 1;

    Ok((x, y))
}

pub fn create_config() -> Result<()> {
    if let Some(home_path) = home_dir() {
        let config_dir = home_path.join(".config/typy");
        let config_file = config_dir.join("config.toml");

        if !config_dir.exists() {
            fs::create_dir_all(&config_dir)
                .context("Failed to create config directory")?;
        }

        if !config_file.exists() {
            fs::File::create(&config_file)
                .context("Failed to create config file")?;
        }
    } else {
        eprintln!("Failed to get home directory");
    }
    Ok(())
}

pub fn open_config() -> Result<()> {
    if let Some(home_path) = home_dir() {
        let config_dir = home_path.join(".config/typy");
        let config_file = config_dir.join("config.toml");

        if !config_file.exists() {
            eprintln!("Config file doesn't exist");
            return Ok(());
        }

        let editor = std::env::var("EDITOR").unwrap_or_else(|_| "vi".to_string());
        Command::new(editor.clone())
            .arg(config_file)
            .status()
            .with_context(|| format!("Failed to open config file with editor: {}", editor))?;

    } else {
        eprintln!("Failed to get home directory");
    }
    Ok(())
}
