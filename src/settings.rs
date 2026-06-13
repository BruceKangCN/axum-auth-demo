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
    pub host: Option<String>,
    pub port: Option<u16>,
}

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ApplicationSettings {
    pub client_id: String,
    // TODO
    #[allow(unused)]
    pub client_secret: String,
    pub jwk_set_url: Option<String>,
}

pub async fn load_settings() -> anyhow::Result<Settings> {
    let content = tokio::fs::read_to_string("config.toml").await?;
    let settings = toml::from_str(&content)?;

    Ok(settings)
}
