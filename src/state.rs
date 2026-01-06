use crate::theme::Theme;
use log;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ThemeState {
    pub current_theme: Theme,
}

#[derive(Debug, Clone)]
pub struct StateManager {
    state_path: PathBuf,
}

impl StateManager {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let state_path = dirs::config_dir()
            .ok_or("Could not determine config directory")?
            .join("kswitch/state.toml");

        Ok(StateManager { state_path })
    }

    pub fn save(&self, theme: &Theme) -> Result<(), Box<dyn Error>> {
        log::debug!("Saving theme state: {}", theme.to_string());

        let state = ThemeState {
            current_theme: theme.clone(),
        };

        let toml_string = toml::to_string(&state)?;

        // Ensure the directory exists
        if let Some(parent) = self.state_path.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(&self.state_path, toml_string)?;
        log::info!("Theme state saved to {}", self.state_path.to_string_lossy());

        Ok(())
    }

    pub fn load(&self) -> Result<ThemeState, Box<dyn Error>> {
        log::debug!(
            "Loading theme state from {}",
            self.state_path.to_string_lossy()
        );

        if !self.state_path.exists() {
            log::debug!("State file does not exist, returning Light theme as default");
            return Ok(ThemeState {
                current_theme: Theme::Light,
            });
        }

        let contents = fs::read_to_string(&self.state_path)?;
        let state: ThemeState = toml::from_str(&contents)?;

        log::info!(
            "Theme state loaded from {}: {}",
            self.state_path.to_string_lossy(),
            state.current_theme.to_string()
        );

        Ok(state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_state_manager_save_and_load() {
        let state_manager = StateManager::new().expect("Failed to create StateManager");
        let theme = Theme::Dark;

        // Save the theme
        let result = state_manager.save(&theme);
        assert!(result.is_ok(), "Failed to save theme state");

        // Load the theme
        let loaded = state_manager.load();
        assert!(loaded.is_ok(), "Failed to load theme state");

        let loaded_state = loaded.unwrap();
        assert_eq!(loaded_state.current_theme, Theme::Dark);

        // Cleanup
        let _ = fs::remove_file(&state_manager.state_path);
    }

    #[test]
    fn test_state_manager_default_when_no_file() {
        // Create a temporary state manager with a non-existent path
        let temp_manager = StateManager {
            state_path: PathBuf::from("/tmp/nonexistent_kswitch_state_12345.toml"),
        };

        let loaded = temp_manager.load();
        assert!(loaded.is_ok(), "Should return Ok with default theme");

        let state = loaded.unwrap();
        assert_eq!(state.current_theme, Theme::Light);
    }

    #[test]
    fn test_state_manager_light_theme() {
        let state_manager = StateManager::new().expect("Failed to create StateManager");
        let theme = Theme::Light;

        let result = state_manager.save(&theme);
        assert!(result.is_ok());

        let loaded = state_manager.load();
        assert!(loaded.is_ok());
        assert_eq!(loaded.unwrap().current_theme, Theme::Light);

        // Cleanup
        let _ = fs::remove_file(&state_manager.state_path);
    }
}
