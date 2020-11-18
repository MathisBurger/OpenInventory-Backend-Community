use crate::utils::installation_functions;
use crate::utils;
use std::process;
use std::borrow::{Borrow, BorrowMut};

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
            println!("Tables werden überprüft");
            let mut conn = utils::MySQL::getConn().await.expect("Cannot get Connection");
            println!("Conn established");
            utils::mysql_installation::checkForTables(&conn).await.expect("ETRjhkhkhijhgsdokf");
            conn.close();
        }
    }

}