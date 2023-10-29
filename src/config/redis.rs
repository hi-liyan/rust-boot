use serde::Deserialize;
use serde_yaml::Value;

pub fn parse_redis_value(value: &Value) -> Option<Redis> {
    let redis = if let Some(it) = value.get("redis") {
        let host = it.get("host")
            .and_then(|it| it.as_str())
            .and_then(|it| Some(it.to_string()));

        let port = it.get("port")
            .and_then(|it| it.as_u64())
            .and_then(|it| Some(u16::try_from(it).unwrap()));

        let pass = it.get("pass")
            .and_then(|it| {
                if it.is_number() {
                    it.as_i64().and_then(|it| Some(it.to_string()))
                } else {
                    it.as_str().and_then(|it| Some(it.to_string()))
                }
            });

        let redis = Redis {
            host,
            port,
            pass,
        };

        Some(redis)
    } else {
        None
    };
    redis
}

#[derive(Debug, Clone, Deserialize)]
pub struct Redis {
    pub host: Option<String>,
    pub port: Option<u16>,
    pub pass: Option<String>,
}