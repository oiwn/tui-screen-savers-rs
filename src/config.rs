use crate::common::DefaultOptions;
use crate::error::{ConfigError, Result, TartsError};
use crate::life::ConwayLife;
use crate::maze::Maze;
use crate::rain::digital_rain::DigitalRain;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct Config {
    #[serde(default)]
    matrix: DigitalRainConfig,
    #[serde(default)]
    life: LifeConfig,
    #[serde(default)]
    maze: MazeConfig,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DigitalRainConfig {
    drops_range: Option<(u16, u16)>,
    speed_range: Option<(u16, u16)>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LifeConfig {
    initial_cells: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MazeConfig {
    // Currently empty as Maze has no configurable options
}

impl Config {
    pub fn load() -> Result<Self> {
        let proj_dirs = ProjectDirs::from("", "", "tarts").ok_or_else(|| {
            TartsError::Config(ConfigError::MissingField(
                "Could not determine config directory".into(),
            ))
        })?;

        let config_path = proj_dirs.config_dir().join("tarts.toml");

        if !config_path.exists() {
            return Ok(Config::default());
        }

        let contents = std::fs::read_to_string(config_path)?;
        toml::from_str(&contents)
            .map_err(|e| TartsError::Config(ConfigError::DeserializeFormat(e)))
    }

    #[cfg(test)]
    pub fn save(&self, path: &std::path::Path) -> Result<()> {
        let contents = toml::to_string(self)
            .map_err(|e| TartsError::Config(ConfigError::SerializeFormat(e)))?;
        std::fs::write(path, contents)?;
        Ok(())
    }
}

impl DigitalRainConfig {
    pub fn to_options(
        &self,
        screen_size: (u16, u16),
    ) -> crate::rain::digital_rain::DigitalRainOptions {
        if self.drops_range.is_none() && self.speed_range.is_none() {
            return DigitalRain::default_options(screen_size.0, screen_size.1);
        }

        let default = DigitalRain::default_options(screen_size.0, screen_size.1);

        crate::rain::digital_rain::DigitalRainOptionsBuilder::default()
            .screen_size(screen_size)
            .drops_range(self.drops_range.unwrap_or_else(|| {
                (
                    default.get_min_drops_number(),
                    default.get_max_drops_number(),
                )
            }))
            .speed_range(self.speed_range.unwrap_or_else(|| {
                (default.get_min_speed(), default.get_max_speed())
            }))
            .build()
            .unwrap()
    }
}

impl LifeConfig {
    pub fn to_options(
        &self,
        screen_size: (u16, u16),
    ) -> crate::life::ConwayLifeOptions {
        if self.initial_cells.is_none() {
            return ConwayLife::default_options(screen_size.0, screen_size.1);
        }

        crate::life::ConwayLifeOptionsBuilder::default()
            .screen_size(screen_size)
            .initial_cells(self.initial_cells.unwrap_or_else(|| {
                ConwayLife::default_options(screen_size.0, screen_size.1)
                    .initial_cells
            }))
            .build()
            .unwrap()
    }
}

impl MazeConfig {
    pub fn to_options(&self, screen_size: (u16, u16)) -> crate::maze::MazeOptions {
        Maze::default_options(screen_size.0, screen_size.1)
    }
}

impl Config {
    pub fn get_matrix_options(
        &self,
        screen_size: (u16, u16),
    ) -> crate::rain::digital_rain::DigitalRainOptions {
        self.matrix.to_options(screen_size)
    }

    pub fn get_life_options(
        &self,
        screen_size: (u16, u16),
    ) -> crate::life::ConwayLifeOptions {
        self.life.to_options(screen_size)
    }

    pub fn get_maze_options(
        &self,
        screen_size: (u16, u16),
    ) -> crate::maze::MazeOptions {
        self.maze.to_options(screen_size)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_config_load_and_save() {
        let dir = tempdir().expect("Unable to create tempdir");
        let config_path = dir.path().join("test_config.toml");

        // Create a test config
        let mut config = Config::default();
        config.matrix.drops_range = Some((100, 200));
        config.matrix.speed_range = Some((5, 20));
        config.life.initial_cells = Some(5000);

        // Save the config
        config
            .save(&config_path)
            .expect("Unable to save config file...");

        // Read the saved content
        let contents = fs::read_to_string(&config_path)
            .expect("Unable to read config content...");
        let loaded_config: Config =
            toml::from_str(&contents).expect("Unable to load config...");

        // Verify the loaded config matches the original
        assert_eq!(loaded_config.matrix.drops_range, Some((100, 200)));
        assert_eq!(loaded_config.matrix.speed_range, Some((5, 20)));
        assert_eq!(loaded_config.life.initial_cells, Some(5000));
    }

    #[test]
    fn test_default_when_no_config() -> Result<()> {
        let config = Config::default();
        let screen_size = (80, 40);

        // Test matrix options
        let matrix_opts = config.get_matrix_options(screen_size);
        let matrix_default =
            DigitalRain::default_options(screen_size.0, screen_size.1);
        assert_eq!(
            matrix_opts.get_min_drops_number(),
            matrix_default.get_min_drops_number()
        );
        assert_eq!(
            matrix_opts.get_max_drops_number(),
            matrix_default.get_max_drops_number()
        );

        // Test life options
        let life_opts = config.get_life_options(screen_size);
        let life_default =
            ConwayLife::default_options(screen_size.0, screen_size.1);
        assert_eq!(life_opts.initial_cells, life_default.initial_cells);

        // Test maze options
        let maze_opts = config.get_maze_options(screen_size);
        let maze_default = Maze::default_options(screen_size.0, screen_size.1);
        assert_eq!(maze_opts.screen_size, maze_default.screen_size);

        Ok(())
    }

    #[test]
    fn test_partial_config() {
        let dir = tempdir().unwrap();
        let config_path = dir.path().join("tarts.toml");

        // Create config with only some values set
        let toml_content = r#"
            [matrix]
            drops_range = [50, 100]

            [life]
            initial_cells = 2000
        "#;
        fs::write(&config_path, toml_content).unwrap();

        let config: Config = toml::from_str(toml_content).unwrap();
        let screen_size = (80, 40);

        // Test matrix options - drops_range should be from config, speed_range from default
        let matrix_opts = config.get_matrix_options(screen_size);
        assert_eq!(matrix_opts.get_min_drops_number(), 50);
        assert_eq!(matrix_opts.get_max_drops_number(), 100);

        let default_matrix =
            DigitalRain::default_options(screen_size.0, screen_size.1);
        assert_eq!(matrix_opts.get_min_speed(), default_matrix.get_min_speed());
        assert_eq!(matrix_opts.get_max_speed(), default_matrix.get_max_speed());

        // Test life options - should use config value
        let life_opts = config.get_life_options(screen_size);
        assert_eq!(life_opts.initial_cells, 2000);
    }

    #[test]
    fn test_full_config() {
        let dir = tempdir().unwrap();
        let config_path = dir.path().join("tarts.toml");

        // Create complete config
        let toml_content = r#"
            [matrix]
            drops_range = [50, 100]
            speed_range = [5, 25]

            [life]
            initial_cells = 2000
        "#;
        fs::write(&config_path, toml_content).unwrap();

        let config: Config = toml::from_str(toml_content).unwrap();
        let screen_size = (80, 40);

        // Test all options come from config
        let matrix_opts = config.get_matrix_options(screen_size);
        assert_eq!(matrix_opts.get_min_drops_number(), 50);
        assert_eq!(matrix_opts.get_max_drops_number(), 100);
        assert_eq!(matrix_opts.get_min_speed(), 5);
        assert_eq!(matrix_opts.get_max_speed(), 25);

        let life_opts = config.get_life_options(screen_size);
        assert_eq!(life_opts.initial_cells, 2000);
    }
}
