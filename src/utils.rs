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
    app_name: String,

    #[serde(skip)]
    db_url: String,
}

//DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@${DB_HOST}:${DB_PORT}/${DB_NAME}
pub fn load_config() -> Result<Config, anyhow::Error> {
    dotenvy::dotenv().with_context(|| "failed to find .env")?;
    let mut config: Config = envy::from_env().with_context(|| "failed to serialize config")?;
    config.db_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        config.db_user, config.db_password, config.db_host, config.db_port, config.app_name
    );
    Ok(config)
}

#[test]
fn config_load_test() {
    let config = load_config().unwrap();
    println!("{}", config.db_url);
}
