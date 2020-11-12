use rand::distributions::Alphanumeric;
use rand::Rng;

pub fn generate() -> String{
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .collect::<String>()
}