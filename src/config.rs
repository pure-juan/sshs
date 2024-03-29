use std::{fs, io};
use std::fs::{DirBuilder, File};
use std::io::Write;
use std::path::{Path, PathBuf};

use home::home_dir;
use log::debug;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("failed to find $HOME folder")]
    ErrFindHomeDir,
    #[error("failed to create folder")]
    ErrCreateDir(#[source] io::Error),
    #[error("failed to create file")]
    ErrCreateFile(#[source] io::Error),
    #[error("failed to write config file")]
    ErrWriteFile,
    #[error("failed to read from file")]
    ErrReadFromFile(#[source] io::Error),
    #[error("failed to parse from file")]
    ErrParseFromFile(#[source] serde_yaml::Error),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Server {
    pub alias: String,
    pub username: String,
    pub host: String,
    pub identity: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub servers: Vec<Server>,
}

pub fn init() -> Result<(), ConfigError> {
    let home_path = match get_home_path() {
        Ok(path) => path,
        Err(err) => return Err(err),
    };

    let root_path = match init_config_folder(home_path.clone()) {
        Ok(path) => path,
        Err(err) => return Err(err),
    };

    init_config_file(root_path)
}

pub fn check() -> bool {
    let home_path = match get_home_path() {
        Ok(path) => path,
        Err(err) => {
            debug!("failed to get $HOME: {err}");
            return false;
        }
    };

    let config_folder_path = get_config_folder_path(home_path.clone());
    debug!("config folder path is {:?}", config_folder_path.clone());

    if !check_config_folder(config_folder_path.clone()) {
        debug!("failed to find config folder");
        return false;
    }
    debug!("check config folder passed");

    let config_file_path = get_config_file_path(config_folder_path.clone());
    debug!("config file path is {:?}", config_file_path.clone());
    if !check_config_file(config_file_path) {
        debug!("failed to find config file");
        return false;
    }
    debug!("check config file passed");
    true
}

pub fn parse() -> Result<Config, ConfigError> {
    let home_path = match get_home_path() {
        Ok(path) => path,
        Err(_) => return Err(ConfigError::ErrFindHomeDir),
    };

    let config_file_path = get_config_file_path(get_config_folder_path(home_path));

    let yaml = match fs::read_to_string(config_file_path) {
        Ok(str) => str,
        Err(err) => return Err(ConfigError::ErrReadFromFile(err)),
    };

    let config: Config = match serde_yaml::from_str(yaml.as_str()) {
        Ok(cfg) => cfg,
        Err(err) => return Err(ConfigError::ErrParseFromFile(err)),
    };

    Ok(config)
}

fn get_config_folder_path(dest: PathBuf) -> PathBuf {
    append_to_path(dest, ".sshs")
}

fn check_config_folder(dest: PathBuf) -> bool {
    Path::new(&dest).exists()
}

fn init_config_folder(dest: PathBuf) -> Result<PathBuf, ConfigError> {
    let config_path = append_to_path(dest, ".sshs");

    let result = DirBuilder::new()
        .recursive(true)
        .create(config_path.clone());

    match result {
        Ok(_) => Ok(config_path),
        Err(err) => Err(ConfigError::ErrCreateDir(err)),
    }
}

fn get_config_file_path(folder_path: PathBuf) -> PathBuf {
    append_to_path(folder_path, "server.yaml")
}

fn check_config_file(file_path: PathBuf) -> bool {
    Path::new(&file_path).exists()
}

fn init_config_file(root_path: PathBuf) -> Result<(), ConfigError> {
    let result = File::create(get_config_file_path(root_path));

    let mut file = match result {
        Ok(f) => f,
        Err(err) => return Err(ConfigError::ErrCreateFile(err)),
    };

    let config = make_default_config();

    let config_str = match serde_yaml::to_string(&config) {
        Ok(str) => str,
        Err(err) => {
            debug!("config file serialize error: {err}");
            return Err(ConfigError::ErrWriteFile);
        }
    };

    match file.write_all(config_str.as_bytes()) {
        Ok(_) => Ok(()),
        Err(err) => {
            debug!("config write err: {err}");
            Err(ConfigError::ErrWriteFile)
        }
    }
}

fn make_default_config() -> Config {
    Config {
        servers: vec!(Server {
            alias: String::from("example-server"),
            username: String::from("example"),
            host: String::from("example"),
            identity: Some("".to_string()),
        }),
    }
}

fn get_home_path() -> Result<PathBuf, ConfigError> {
    match home_dir() {
        Some(path) => Ok(path),
        None => Err(ConfigError::ErrFindHomeDir),
    }
}

fn append_to_path(mut p: PathBuf, s: &str) -> PathBuf {
    p.push(s);
    p
}
