use config::{Config, ConfigError, File};

#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        let s = format!(
            "postgres://{0}:{1}@{2}:{3}/{4}",
            self.username, self.password, self.host, self.port, self.database_name
        );
        s
    }

    /// Connection without logical database name, to connect
    /// to the postgres instance, not to the logical database
    pub fn connection_string_without_db(&self) -> String {
        let s = format!(
            "postgres://{}:{}@{}:{}",
            self.username, self.password, self.host, self.port
        );
        s
    }
}

pub fn get_configuration() -> Result<Settings, ConfigError> {
    let settings = Config::builder()
        .add_source(File::with_name("configuration"))
        .build()
        .unwrap();
    settings.try_deserialize::<Settings>()
}
