use std::process;
use crate::utils::installation_functions;

pub async fn loadParam(path: String) -> std::io::Result<String> {
    let config = installation_functions::getContent().await;
    let data: serde_json::Value = serde_json::from_str(&config).expect("Cannot read json from string");
    let split = path.split(&";");
    let params = split.collect::<Vec<&str>>();
    let mut value = "";
    if params.len() == 1 {
        value = data[params[0]].as_str().unwrap();
    } else if params.len() == 2 {
        value = data[params[0]][params[1]].as_str().unwrap();
    } else {
        println!("Index of params array out of range");
        process::exit(0);
    }
    Ok(value.to_string())

}