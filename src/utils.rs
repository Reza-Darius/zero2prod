use anyhow::Context;
use serde::Deserialize;

// the envy crate assumes lower case field names for upper case env variables, it will not compile
// otherwise
#[derive(Debug, Deserialize)]
pub struct Config {
    db_user: String,
    db_password: String,
    db_port: u16,
    db_host: String,
    db_name: String,
}

impl Config {
    pub fn db_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.db_user,
            self.db_password,
            self.db_host,
            self.db_port,
            self.db_name
        )
    }
}

pub fn load_config() -> Result<Config, anyhow::Error> {
    dotenvy::dotenv().with_context(|| "failed to find .env")?;
    envy::from_env().with_context(|| "failed to serialize config")
}

#[test]
fn config_load_test() {
    let config = load_config().unwrap();
    println!("{}", config.db_url());
}
