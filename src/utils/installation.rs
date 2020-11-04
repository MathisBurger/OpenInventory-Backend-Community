use crate::utils::installation_functions;

pub fn InstallationProcess() {
    let configInstallation = installation_functions::check_available();
    println!("Datei existiert: {}", configInstallation);

}