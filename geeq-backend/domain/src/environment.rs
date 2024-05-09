use std::str::FromStr;
use std::env;

use once_cell::sync::Lazy;

#[derive(Debug, PartialEq)]
pub enum Environment {
    Prod,
    Local,
}


impl FromStr for Environment {
    type Err = ();
    fn from_str(input: &str) -> Result<Environment, Self::Err> {
        match input {
            "prod"  => Ok(Environment::Prod),
            "local" => Ok(Environment::Local),
            _       => Err(()),
        }
    }
}

pub static ENV: Lazy<Environment> = Lazy::new(|| {
    let env_str = env::var("ENV").expect("env variable is not found");
    return Environment::from_str(&env_str).expect("env variable is invalid");
});