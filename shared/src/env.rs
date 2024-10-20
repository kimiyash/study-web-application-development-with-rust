use std::env;
use strum::EnumString;

#[derive(Default, EnumString)]
pub enum Environment {
    #[default]
    Development,
    Production,
}

pub fn which() -> Environment {
    #[cfg(debug_assertions)]
    let defautl_env = Environment::Development;
    #[cfg(not(debug_assertions))]
    let defautl_env = Environment::Production;

    match env::var("ENV") {
        Err(_) => defautl_env,
        Ok(v) => v.parse().unwrap_or(defautl_env),
    }
}
