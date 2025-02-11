use chrono::Local;
use rand::distributions::Alphanumeric;
use rand::Rng;

pub fn generate_random_string(length: usize) -> String {
    let random_string: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect();
    random_string
}

pub fn formatted_date(format: &str) -> String {
    let now = Local::now();
    now.format(format).to_string()
}