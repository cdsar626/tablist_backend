use config::{Config, File};

#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub host: String,
    pub database_name: String,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let mut settings = Config::default();
    settings.merge(File::with_name("configuration"))?;
    settings.try_into()
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!("mongodb+srv://{}:{}@{}/{}?retryWrites=true&w=majority",
                self.username,
                self.password,
                self.host,
                self.database_name,
        )
    }

    pub fn connection_string_without_db(&self) -> String {
        format!(
            "mongodb+srv://{}:{}@{}/?retryWrites=true&w=majority",
            self.username,
            self.password,
            self.host,
        )
    }
}