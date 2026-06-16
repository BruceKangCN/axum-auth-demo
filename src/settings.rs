use serde::Deserialize;

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Settings {
    pub server: ServerSettings,
    pub app: ApplicationSettings,
}

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ServerSettings {
    #[serde(default = "ServerSettings::default_host")]
    pub host: String,
    #[serde(default = "ServerSettings::default_port")]
    pub port: u16,
}

impl ServerSettings {
    fn default_host() -> String {
        "localhost".to_string()
    }

    fn default_port() -> u16 {
        3000
    }
}

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ApplicationSettings {
    pub slug: String,
    pub client_id: String,
    pub client_secret: String,
    #[serde(default = "ApplicationSettings::default_authentik_base_url")]
    pub authentik_base_url: String,
    pub redirect_uri: String,
}

impl ApplicationSettings {
    fn default_authentik_base_url() -> String {
        "http://localhost:9000".to_string()
    }
}

pub async fn load_settings() -> anyhow::Result<Settings> {
    let content = tokio::fs::read_to_string("config.toml").await?;
    let settings = toml::from_str(&content)?;

    Ok(settings)
}
