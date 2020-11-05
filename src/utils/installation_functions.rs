use std::path::Path;
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
use serde_json::Value;

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

pub async fn check_config_syntax(config: String) -> std::io::Result<()> {
    let data: serde_json::Value = serde_json::from_str(&config).unwrap();
    let obj = data.as_object().unwrap();
    let db_value = obj.get("database").unwrap();
    println!("{}", db_value);
    Ok(())
}

