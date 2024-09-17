use std::env;

pub fn load_config() -> String {
    env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}