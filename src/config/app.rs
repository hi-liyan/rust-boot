use serde::Deserialize;
use serde_yaml::Value;

pub fn parse_app_value(value: &Value) -> Option<App> {
    let app = if let Some(it) = value.get("app") {
        let name = it.get("name")
            .and_then(|it| it.as_str())
            .and_then(|it| Some(it.to_string()));

        let version = it.get("version")
            .and_then(|it| it.as_str())
            .and_then(|it| Some(it.to_string()));


        let port = it.get("port")
            .and_then(|it| it.as_i64())
            .and_then(|it| Some(u16::try_from(it).unwrap()));

        let app = App {
            name,
            version,
            port,
        };
        Some(app)
    } else {
        None
    };
    app
}

#[derive(Debug, Clone, Deserialize)]
pub struct App {
    pub name: Option<String>,
    pub version: Option<String>,
    pub port: Option<u16>,
}