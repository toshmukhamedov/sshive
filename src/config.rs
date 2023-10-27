use serde::Deserialize;
use std::{
    fs,
    io::{self, Write},
    path, process,
};

pub struct Config {
    pub value: ConfigValue,
}

#[derive(Deserialize)]
pub struct ConfigValue {
    pub connections: Vec<Connection>,
}

#[derive(Deserialize, Debug)]
pub struct Connection {
    pub user: String,
    pub host: String,
    pub port: Option<u16>,
    pub tag: String,
    pub identity_file: path::PathBuf,
}
impl Default for Config {
    fn default() -> Self {
        Self { value: Self::read() }
    }
}

impl Config {
    fn read() -> ConfigValue {
        let logger = crate::logger::Logger::default();
        let mut config_path = match std::env::var("HOME") {
            Ok(dir) => path::PathBuf::from(dir),
            Err(_) => {
                panic!("Unable to determine user directory");
            },
        };

        // Create directories
        for dir in [".config", "sshive"] {
            config_path.push(dir);
            if let Err(err) = fs::metadata(&config_path) {
                if err.kind() == io::ErrorKind::NotFound {
                    // Directory doesn't exist, create it
                    if let Err(create_err) = fs::create_dir(&config_path) {
                        panic!("Failed to create directory: {}", create_err);
                    }
                    logger.debug(format!("Directory created: {}", config_path.display()));
                    continue;
                }
                panic!("Problem reading: {}", config_path.display())
            }
        }

        config_path.push("config.json");
        let file = fs::File::open(&config_path).unwrap_or_else(|error| match error.kind() {
            io::ErrorKind::NotFound => {
                logger.debug(format!("Error while opening config from {}", config_path.display()));

                let mut file = std::fs::File::create(&config_path).unwrap();
                file.write_all(b"{\"connections\": []}").unwrap();
                file
            },
            _ => panic!("Problem opening the file: {:?}", error),
        });

        let reader = std::io::BufReader::new(file);
        serde_json::from_reader(reader).unwrap_or_else(|err| {
            eprintln!("Error while reading config: {}", err);
            process::exit(0);
        })
    }
}
