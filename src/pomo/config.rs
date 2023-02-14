use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::{Read, Write},
    path::PathBuf,
};
use toml;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Config {
    pub focus_duration: u64,
    pub short_break_duration: u64,
    pub long_break_duration: u64,
    pub long_break_after: u64,
}

impl Config {
    pub fn init(cfg_base_path: PathBuf) -> std::io::Result<()> {
        let cfg_file_path = cfg_base_path.join("config.toml");

        fs::create_dir_all(cfg_base_path)?;

        let mut cfg_file = fs::File::create(cfg_file_path)?;
        let cfg = Self::default();
        let toml_cfg = toml::to_string(&cfg).unwrap();
        let toml_cfg = toml_cfg.as_bytes();

        cfg_file.write_all(toml_cfg)?;
        Ok(())
    }

    pub fn load(cfg_base_path: PathBuf) -> std::io::Result<Self> {
        let cfg_file_path = cfg_base_path.join("config.toml");
        let mut file = fs::File::open(cfg_file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let cfg: Self = toml::from_str(contents.as_ref()).unwrap();

        Ok(cfg)
    }

    pub fn default() -> Self {
        Self {
            focus_duration: 25,
            short_break_duration: 5,
            long_break_duration: 25,
            long_break_after: 4,
        }
    }
}
