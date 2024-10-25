use std::env;

pub fn get(var_name: &str) -> String {
    env::var(var_name).unwrap_or_else(|_| panic!("{} doit être défini dans .env", var_name))
}
