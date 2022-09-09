use std::{collections::HashMap, fs::{self, read_dir}, io::Write};
use std::fs::File;
use std::path::Path;

use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Deserialize, Serialize)]
pub(crate) struct Config {
    token: String,
    owner: u64,
    max_guilds: usize,
    firebase_config: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct LogInfo {
    joins: bool,
    leaves: bool,
    edit: bool,
    delete: bool,
    log_channel: Option<u64>,
}

#[allow(dead_code)]
#[derive(Deserialize, Serialize)]
pub(crate) struct AutoReaction {
    emoji: String,
    channel: u64,
}

#[allow(dead_code)]
#[derive(Deserialize, Serialize)]
pub(crate) struct ServerConfig {
    guild: u64,
    log: LogInfo,
    autoreactions: Vec<AutoReaction>,
    control: bool,
    mute_role: Option<u64>,
    edit_snipe: Option<String>,
    ghost_ping: bool,
    staff_role: Option<u64>,
}

impl Default for ServerConfig {
    fn default() -> Self {
        unimplemented!();
    }
}

impl ServerConfig {
    pub(crate) fn load() -> HashMap<u64, Self> {
        // dir doesn't exist
        let servers_dir = read_dir("servers");
        if servers_dir.is_err() {
            let _ = fs::create_dir_all("servers");
            return HashMap::new();
        }
        let mut map = HashMap::new();
        let servers_dir = servers_dir.unwrap();
        for dir in servers_dir {
            let dir = dir.unwrap();
            let content = fs::read_to_string(dir.path()).unwrap();
            let data: ServerConfig = serde_json::from_str(&content).unwrap();
            map.insert(data.guild, data);
        }
        map
    }
    pub(crate) fn create(guild: u64) -> Result<(), String> {
        let servers_dir = read_dir("servers");
        if servers_dir.is_err() {
            let _ = fs::create_dir_all("servers");
            return Err(String::from("unable to read servers directory, failed to create server config file"));
        }
        let path = format!("servers/{guild}.json");
        if Path::new(&path).exists() {
            return Err(String::from("config file already exists, do you want to clear it? use /clearconfig"));
        }
        // check if file exists
        let mut file = File::create(path).unwrap();
        let _ = file.set_len(0);
        file.write_all(serde_json::to_string_pretty(&ServerConfig::default()).unwrap().as_bytes()).unwrap();
        Ok(())
    }
    pub(crate) fn remove(guild: u64) -> Result<(), String> {
        let path = format!("servers/{guild}.json");
        if Path::new(&path).exists() {
            return Err(String::from("config file does not exist! nothing to clear"));
        }
        Ok(())
    }
}
