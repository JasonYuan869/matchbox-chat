use std::net::IpAddr;
use config::Case;
use serde::Deserialize;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::filter::LevelFilter;

#[derive(Deserialize, Debug)]
pub(crate) struct Settings {
    bind_address: IpAddr,
    port: u16,
}

pub fn get_configuration_from_env() -> Result<Settings, config::ConfigError> {
    let env = config::Environment::with_convert_case(Case::ScreamingSnake);

    let settings = config::Config::builder()
        .add_source(env)
        .build()?;

    settings.try_deserialize::<Settings>()
}