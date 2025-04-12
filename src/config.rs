use serde::{Deserialize, Serialize};

pub static CONFIG_FILE_NAME: &str = "bread_config.toml";

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub serve_at: ServeAt,
    pub mislead: Mislead,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            serve_at: ServeAt::default(),
            mislead: Mislead::default(),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct ServeAt {
    pub ip: String,
    pub port: u16,
}

impl Default for ServeAt {
    fn default() -> Self {
        ServeAt {
            ip: String::from("127.0.0.1"),
            port: 8080,
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Mislead {
    pub link_to_mislead: String,
    pub mislead_to: String,
}

impl Default for Mislead {
    fn default() -> Self {
        Mislead {
            link_to_mislead: String::from("http://127.0.0.1:8080"),
            mislead_to: String::from("http://127.0.0.1:8080"),
        }
    }
}
