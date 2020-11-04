use crate::utils::installation_functions;
use std::process;


pub async fn InstallationProcess() {
    if installation_functions::check_available() {
        println!("Configuration file does not exist");
        if installation_functions::create_file() {
            println!("Successfully created configuration file");
        } else {
            println!("Cannot create configuration file");
            process::exit(0);
        }
    }
    if installation_functions::getContent().is_empty() {
        println!("Configuration file is empty");
        let config_template = installation_functions::getConfigTemplate();
    }

}