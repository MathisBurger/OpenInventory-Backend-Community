use std::path::PathBuf;
use std::io;
use std::fs;
use json::JsonValue;
use curl::easy::Easy;
use std::io::prelude::*;
use serde_json;
use std::env;
use std::io::BufReader;
use std::fs::File;
use serde_json::{Value, to_string};

pub fn check_available() -> bool {
    fs::metadata("./config.json").is_ok()
}

pub fn create_file() -> bool {
    let file = fs::File::create(&"./config.json");
    check_available()
}

pub async fn getContent() -> String {
    let mut data = String::new();
    let mut f = File::open(&"./config.json").expect("Unable to open file");
    f.read_to_string(&mut data).expect("Unable to read string");
    return data;
}

pub fn getConfigTemplate() -> String {
    let mut data = Vec::new();
    let mut handle = Easy::new();
    handle.url("https://mathis-burger.de/mirror/OpenInventory/config.json").unwrap();
    {
        let mut transfer = handle.transfer();
        transfer.write_function(|new_data| {
            data.extend_from_slice(new_data);
            Ok(new_data.len())
        }).unwrap();
        transfer.perform().unwrap();
    }
    String::from_utf8(data).expect("body is not valid UTF8!")
}

pub fn write_default_config(config: String) -> std::io::Result<()> {
    fs::write("./config.json", config.as_bytes());
    Ok(())
}

pub fn check_config_syntax(config: String) -> std::io::Result<String> {
    let data: serde_json::Value = serde_json::from_str(&config).expect("Cannot read json from string");
    if data["database"]["username"].as_str().unwrap() != "<username>" {
        if data["database"]["password"].as_str().unwrap() != "<password>" {
            if data["database"]["database"].as_str().unwrap() != "<database>" {
                if data["database"]["host"].as_str().unwrap() != "<host>" {
                    Ok("Config contains no errors".to_string())
                } else {
                    Ok("Host cant be default. Please change it".to_string())
                }
            } else {
                Ok("Database name cant be default. Please Change it".to_string())
            }
        } else {
            Ok("Password cant be default. Please change it".to_string())
        }
    } else {
        Ok("Username cant be template default. Please change it".to_string())
    }
}

