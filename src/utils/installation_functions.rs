use std::path::Path;
use std::path::PathBuf;
use std::{fs, io};
use json::JsonValue;
use curl::easy::Easy;

pub fn check_available() -> bool {
    let path = Path::new(&"./config.json");
    path.exists()
}

pub fn create_file() -> bool {
    let file = fs::File::create(&"./config.json");
    check_available()
}

pub fn getContent() -> String {
    let contents = fs::read_to_string("./config.json")
        .expect("error while reading file");
    return contents;
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

