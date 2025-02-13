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

pub fn generate_id(name: &str) -> String {
    let clean_name: String = name
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '_' })
        .collect();
    format!("XXXX____id_{}_{}_{}_{}", clean_name, formatted_date("%Y_%m_%d_%H_%M_%S"),generate_random_string(10), "____XXXX")
}
