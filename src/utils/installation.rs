use crate::utils::installation_functions;
use crate::utils;
use std::process;

pub async fn InstallationProcess() {
    if !installation_functions::check_available() {
        println!("Configuration file does not exist");
        if installation_functions::create_file() {
            println!("Successfully created configuration file");
        } else {
            println!("Cannot create configuration file");
            process::exit(0);
        }
    }
    if installation_functions::getContent().await.is_empty() {
        println!("Configuration file is empty");
        let config_template = installation_functions::getConfigTemplate();
        installation_functions::write_default_config(config_template);
        println!("Wrote default config");
        println!("Please edit configuration.");
        process::exit(0);
    } else {
        let config = installation_functions::getContent().await;
        let ans = installation_functions::check_config_syntax(config).expect("Errow while processing config");
        if ans != "Config contains no errors".to_string() {
            println!("{}", ans);
            process::exit(0);
        } else {
            let para: String = format!("{}", "database;username");
            let param = utils::loadFromConfig::loadParam(para).await.expect("Error while loading param from config");
            println!("{}", param);
        }
    }

}