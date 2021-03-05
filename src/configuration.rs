use mongodb::options::{ClientOptions, Credential};
use mongodb::{Client, Database};
use serde_aux::field_attributes::deserialize_number_from_string;
use std::convert::{TryFrom, TryInto};
use std::fs::File;
use std::io::BufReader;

#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application: ApplicationSettings,
    pub firebase: FirebaseSettings,
}

#[derive(serde::Deserialize)]
pub struct ApplicationSettings {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    uri: String,
    user: String,
    password: String,
    name: String,
}

impl DatabaseSettings {
    pub async fn db(self) -> Result<Database, mongodb::error::Error> {
        let mut client_options: ClientOptions = ClientOptions::parse(&self.uri).await?;
        client_options.credential = Some(
            Credential::builder()
                .username(Some(self.user))
                .password(Some(self.password))
                .build(),
        );
        let database: Database = Client::with_options(client_options)?.database(&self.name);
        Ok(database)
    }
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct FirebaseConfig {
    pub project_id: String,
    pub private_key_id: String,
    pub private_key: String,
    pub client_email: String,
    pub client_id: String,
}

impl FirebaseConfig {
    pub fn new(path: &str) -> FirebaseConfig {
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        let config: FirebaseConfig = serde_json::from_reader(reader).unwrap();
        config
    }
}

#[derive(serde::Deserialize)]
pub struct FirebaseSettings {
    pub secret_path: String,
}

pub fn get_configurations() -> Result<Settings, config::ConfigError> {
    let mut settings = config::Config::default();
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let configuration_directory = base_path.join("configuration");

    settings.merge(config::File::from(configuration_directory.join("base")).required(true))?;

    let environment: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to parse APP_ENVIRONMENT.");

    settings.merge(
        config::File::from(configuration_directory.join(environment.as_str())).required(true),
    )?;

    settings.merge(config::Environment::with_prefix("app").separator("__"))?;

    settings.try_into()
}

pub enum Environment {
    Local,
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Production => "production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            other => Err(format!(
                "{} is not a supported environment. Use either `local` or `production`.",
                other
            )),
        }
    }
}
