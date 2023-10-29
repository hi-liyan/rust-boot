use serde::Deserialize;
use serde_yaml::Value;

pub fn parse_encrypt_value(value: &Value) -> Option<Encrypt> {
    let encrypt = if let Some(it) = value.get("encrypt") {
        let key = it.get("key")
            .and_then(|it| it.as_str())
            .and_then(|it| Some(it.to_string()));

        let encrypt = Encrypt {
            key
        };
        Some(encrypt)
    } else {
        None
    };
    encrypt
}

#[derive(Debug, Clone, Deserialize)]
pub struct Encrypt {
    pub key: Option<String>,
}