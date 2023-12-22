use serde::Deserialize;
use serde_yaml::Value;

pub fn parse_smtp_value(value: &Value) -> Option<Smtp> {
    let smtp = if let Some(it) = value.get("smtp") {
        let host = it.get("host")
            .and_then(|it| it.as_str())
            .and_then(|it| Some(it.to_string()));

        let port = it.get("port")
            .and_then(|it| it.as_i64())
            .and_then(|it| Some(u16::try_from(it).unwrap()));

        let user = it.get("user")
            .and_then(|it| it.as_str())
            .and_then(|it| Some(it.to_string()));

        let pass = it.get("pass")
            .and_then(|it| it.as_str())
            .and_then(|it| Some(it.to_string()));

        let max_size = it.get("max_size")
            .and_then(|it| it.as_i64())
            .and_then(|it| Some(u32::try_from(it).unwrap()));

        let min_idle = it.get("min_idle")
            .and_then(|it| it.as_i64())
            .and_then(|it| Some(u32::try_from(it).unwrap()));

        let idle_timeout = it.get("idle_timeout")
            .and_then(|it| it.as_i64())
            .and_then(|it| Some(u64::try_from(it).unwrap()));

        let s = Smtp {
            host,
            port,
            user,
            pass,
            max_size,
            min_idle,
            idle_timeout
        };
        Some(s)
    } else {
        None
    };
    smtp
}

#[derive(Debug, Clone, Deserialize)]
pub struct Smtp {
    pub host: Option<String>,
    pub port: Option<u16>,
    pub user: Option<String>,
    pub pass: Option<String>,
    pub max_size: Option<u32>,
    pub min_idle: Option<u32>,
    pub idle_timeout: Option<u64>
}