use crate::{
    boids::{BoidsOptions, BoidsOptionsBuilder},
    cube::{CubeOptions, CubeOptionsBuilder},
    error::{ConfigError, Result, TartsError},
    life::{ConwayLifeOptions, ConwayLifeOptionsBuilder},
    maze::{MazeOptions, MazeOptionsBuilder},
    rain::digital_rain::{DigitalRainOptions, DigitalRainOptionsBuilder},
};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    #[serde(default)]
    matrix: DigitalRainOptions,
    #[serde(default)]
    life: ConwayLifeOptions,
    #[serde(default)]
    maze: MazeOptions,
    #[serde(default)]
    boids: BoidsOptions,
    #[serde(default)]
    cube: CubeOptions,
}

impl Config {
    // Generate and save default config
    pub fn save_default_config() -> Result<()> {
        let proj_dirs = ProjectDirs::from("", "", "tarts").ok_or_else(|| {
            TartsError::Config(ConfigError::MissingField(
                "Could not determine config directory".into(),
            ))
        })?;

        // Create config directory if it doesn't exist
        let config_dir = proj_dirs.config_dir();
        std::fs::create_dir_all(config_dir)?;

        let config_path = config_dir.join("tarts.toml");

        // Create default config
        let default_config = Config::default();

        // Convert to TOML and save
        let contents = toml::to_string(&default_config)
            .map_err(|e| TartsError::Config(ConfigError::SerializeFormat(e)))?;
        std::fs::write(config_path, contents)?;

        Ok(())
    }

    // Modify your existing load method to create default if missing
    pub fn load_old() -> Result<Self> {
        let proj_dirs = ProjectDirs::from("", "", "tarts").ok_or_else(|| {
            TartsError::Config(ConfigError::MissingField(
                "Could not determine config directory".into(),
            ))
        })?;

        let config_path = proj_dirs.config_dir().join("tarts.toml");

        if !config_path.exists() {
            // Create config directory if it doesn't exist
            std::fs::create_dir_all(proj_dirs.config_dir())?;

            // Create and save default config
            let default_config = Config::default();
            let contents = toml::to_string(&default_config)
                .map_err(|e| TartsError::Config(ConfigError::SerializeFormat(e)))?;
            std::fs::write(&config_path, contents)?;
            return Ok(default_config);
        }

        let contents = std::fs::read_to_string(config_path)?;
        toml::from_str(&contents)
            .map_err(|e| TartsError::Config(ConfigError::DeserializeFormat(e)))
    }

    pub fn load() -> Result<Self> {
        let proj_dirs = ProjectDirs::from("", "", "tarts").ok_or_else(|| {
            eprintln!("Failed to get project directory");
            TartsError::Config(ConfigError::MissingField(
                "Could not determine config directory".into(),
            ))
        })?;

        println!("Config dir: {:?}", proj_dirs.config_dir());
        let config_path = proj_dirs.config_dir().join("tarts.toml");
        println!("Config path: {:?}", config_path);

        if !config_path.exists() {
            println!("Config file doesn't exist, creating default");

            // Create config directory if it doesn't exist
            match std::fs::create_dir_all(proj_dirs.config_dir()) {
                Ok(_) => println!("Created config directory"),
                Err(e) => println!("Failed to create config directory: {}", e),
            }

            // Create default config using builders explicitly
            let default_config = Config {
                matrix: DigitalRainOptionsBuilder::default().build().unwrap(),
                life: ConwayLifeOptionsBuilder::default().build().unwrap(),
                maze: MazeOptionsBuilder::default().build().unwrap(),
                boids: BoidsOptionsBuilder::default().build().unwrap(),
                cube: CubeOptionsBuilder::default()
                    .cube_size(5.0)
                    .rotation_speed_x(0.5)
                    .rotation_speed_y(0.7)
                    .rotation_speed_z(0.3)
                    .distance(3.0)
                    .use_braille(true)
                    .build()
                    .unwrap(),
            };

            println!("Default cube options: {:?}", default_config.cube);

            // Serialize and write
            let contents = toml::to_string(&default_config).map_err(|e| {
                println!("Failed to serialize config: {}", e);
                TartsError::Config(ConfigError::SerializeFormat(e))
            })?;

            println!("TOML contents: {}", contents);

            std::fs::write(&config_path, &contents)?;
            println!("Wrote config file successfully");

            return Ok(default_config);
        }

        println!("Config file exists, loading it");
        let contents = std::fs::read_to_string(&config_path)?;
        println!("Loaded contents: {}", contents);

        let config = toml::from_str(&contents).map_err(|e| {
            println!("Failed to parse config: {}", e);
            TartsError::Config(ConfigError::DeserializeFormat(e))
        })?;

        println!("Parsed config successfully");
        Ok(config)
    }
}

impl Config {
    // Add these methods
    pub fn get_matrix_options(
        &self,
        _screen_size: (u16, u16),
    ) -> DigitalRainOptions {
        // If default is needed, create it, otherwise use stored config
        self.matrix.clone()
    }

    pub fn get_life_options(&self, _screen_size: (u16, u16)) -> ConwayLifeOptions {
        self.life.clone()
    }

    pub fn get_maze_options(&self, _screen_size: (u16, u16)) -> MazeOptions {
        self.maze.clone()
    }

    pub fn get_boids_options(&self, screen_size: (u16, u16)) -> BoidsOptions {
        let mut options = self.boids.clone();
        options.screen_size = screen_size;
        options
    }

    pub fn get_cube_options(&self) -> CubeOptions {
        self.cube.clone()
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            matrix: DigitalRainOptionsBuilder::default().build().unwrap(),
            life: ConwayLifeOptionsBuilder::default().build().unwrap(),
            maze: MazeOptionsBuilder::default().build().unwrap(),
            boids: BoidsOptionsBuilder::default().build().unwrap(),
            cube: CubeOptionsBuilder::default().build().unwrap(),
        }
    }
}
