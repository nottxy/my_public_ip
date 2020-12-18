use serde::Deserialize;
use std::{collections::HashMap, env, fs};

use crate::error::Result;

#[derive(Deserialize)]
pub struct Config {
    readers: HashMap<String, String>,
    writers: HashMap<String, String>,
}

#[derive(Clone)]
pub struct ConfigKeys {
    reader_keys: HashMap<String, String>,
    writer_keys: HashMap<String, String>,
}

impl ConfigKeys {
    pub fn get_reader_name(&self, key: &str) -> Option<&str> {
        self.reader_keys.get(key).map(String::as_str)
    }

    pub fn get_writer_name(&self, key: &str) -> Option<&str> {
        self.writer_keys.get(key).map(String::as_str)
    }
}

impl Config {
    pub(crate) fn load(path: &str) -> Result<Config> {
        let content = fs::read_to_string(path)?;
        toml::from_str(&content).map_err(Into::into)
    }
}

impl Default for Config {
    fn default() -> Config {
        let file_path =
            env::var("MY_PUBLIC_IP_CONFIG").expect("the MY_PUBLIC_IP_CONFIG var in env is missing");
        Config::load(&file_path).expect("could not init default config")
    }
}

impl Into<ConfigKeys> for Config {
    fn into(self) -> ConfigKeys {
        let Config { readers, writers } = self;
        let reader_keys = switch_kv(readers);
        let writer_keys = switch_kv(writers);

        ConfigKeys {
            reader_keys,
            writer_keys,
        }
    }
}

fn switch_kv(map: HashMap<String, String>) -> HashMap<String, String> {
    let mut new_map = HashMap::with_capacity(map.len());
    for (k, v) in map {
        if let Some(prev_v) = new_map.insert(v, k) {
            panic!("Key of {} is duplicate", prev_v);
        }
    }
    new_map
}
