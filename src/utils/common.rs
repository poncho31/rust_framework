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

// pub fn reverse_string(input: &str) -> String {
//     input.chars().rev().collect()
// }