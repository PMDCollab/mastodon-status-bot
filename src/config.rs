use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs::read_to_string;
use valuable::Valuable;

#[derive(Debug, Deserialize, Clone, Copy, Valuable)]
#[serde(rename_all = "UPPERCASE")]
pub enum AlertKind {
    Triggered,
    Resolved,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct TemplateEntry {
    down_template: String,
    up_template: String,
}

impl TemplateEntry {
    pub fn get(&self, kind: AlertKind) -> &str {
        match kind {
            AlertKind::Triggered => &self.down_template,
            AlertKind::Resolved => &self.up_template,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Service {
    pub friendly_name: Option<String>,
    template: Option<TemplateEntry>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct TemplateConfig {
    pub default: TemplateEntry,
    pub default_friendly: TemplateEntry,
    service: HashMap<String, HashMap<String, Service>>,
}

impl TemplateConfig {
    pub fn service(&self, group: &str, name: &str) -> Option<&Service> {
        match self.service.get(group) {
            None => None,
            Some(names) => names.get(name),
        }
    }
    pub fn template_for(&self, group: &str, name: &str) -> &TemplateEntry {
        self.service(group, name)
            .and_then(|service| match &service.friendly_name {
                None => service.template.as_ref(),
                Some(_) => match &service.template {
                    None => Some(&self.default_friendly),
                    Some(tpl) => Some(&tpl),
                },
            })
            .unwrap_or(&self.default)
    }
}

#[derive(Debug)]
pub struct Config {
    pub host: String,
    pub client_id: String,
    pub client_secret: String,
    pub token: String,
    pub live: bool,
    pub tpl_config: TemplateConfig,
}

pub fn init() -> Result<Config, Box<dyn Error>> {
    Ok(Config {
        host: read_env("MSB_HOST")?,
        client_id: read_env("MSB_CLIENT_KEY")?,
        client_secret: read_env("MSB_CLIENT_SECRET")?,
        token: read_env("MSB_ACCESS_TOKEN")?,
        live: read_env("MSB_LIVE")
            .map(|s| &s == "true")
            .unwrap_or_default(),
        tpl_config: toml::from_str(&read_to_string(read_env("MSB_CONFIG_FILE")?)?)?,
    })
}

pub fn read_env(name: &str) -> Result<String, Box<dyn Error>> {
    env::var_os(name)
        .map(|s| s.to_string_lossy().to_string())
        .ok_or_else(|| format!("Environment variable {name} must be set.").into())
}
