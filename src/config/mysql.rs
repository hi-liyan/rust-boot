use serde::Deserialize;
use serde_yaml::Value;

pub fn parse_mysql_value(value: &Value) -> Option<Mysql> {
    let mysql = if let Some(it) = value.get("mysql") {
        let host = it.get("host")
            .and_then(|it| it.as_str())
            .and_then(|it| Some(it.to_string()));

        let user = it.get("user")
            .and_then(|it| it.as_str())
            .and_then(|it| Some(it.to_string()));

        let pass = it.get("pass")
            .and_then(|it| {
                if it.is_number() {
                    it.as_i64().and_then(|it| Some(it.to_string()))
                } else {
                    it.as_str().and_then(|it| Some(it.to_string()))
                }
            });

        let port = it.get("port")
            .and_then(|it| it.as_i64())
            .and_then(|it| Some(u16::try_from(it).unwrap()));

        let db_name = it.get("db_name")
            .and_then(|it| it.as_str())
            .and_then(|it| Some(it.to_string()));

        let max_connections = it.get("max_connections")
            .and_then(|it| it.as_i64())
            .and_then(|it| Some(u32::try_from(it).unwrap()));

        let db = Mysql {
            host,
            user,
            pass,
            port,
            db_name,
            max_connections,
        };
        Some(db)
    } else {
        None
    };
    mysql
}

#[derive(Debug, Clone, Deserialize)]
pub struct Mysql {
    pub host: Option<String>,
    pub user: Option<String>,
    pub pass: Option<String>,
    pub port: Option<u16>,
    pub db_name: Option<String>,
    pub max_connections: Option<u32>,
}