use serde_derive::Deserialize;

use std::fs::File;
use std::io::prelude::*;

#[derive(Deserialize)]
#[derive(Debug)]
pub struct Config {
    pub mysql:MysqlDB,
    pub http:Http,
}

#[derive(Deserialize)]
#[derive(Debug)]
pub struct MysqlDB {
    pub host: String,
    pub port: String,
    pub database: String,
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
#[derive(Debug)]
pub struct Http {
    pub host: String,
    pub port: String,
}

pub fn load_config()-> Result<Config,Box<dyn std::error::Error>>{
    let file_path = "config.toml";
    let mut file = File::open(file_path)?;
    let mut str_val = String::new();

    file.read_to_string(&mut str_val)?;

    let config : Config = toml::from_str(&str_val)?;

    Ok(config)
}