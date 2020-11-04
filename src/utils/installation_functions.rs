use std::path::Path;
use std::fs;
pub struct ConfigInstallation {
}

pub fn check_available() -> bool {
    let path = Path::new(&"./config.json");
    path.exists()
}

pub fn create_file() -> bool {
    let file = fs::File::create(&"./config.json");
    check_available()
}