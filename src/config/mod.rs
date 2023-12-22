use std::{fs, process};
use std::error::Error;
use std::path::Path;

use serde::Deserialize;
use serde_yaml::Value;
use tokio::sync::OnceCell;

use crate::config::app::{App, parse_app_value};
use crate::config::encrypt::{Encrypt, parse_encrypt_value};
#[cfg(feature = "feat-mysql")]
use crate::config::mysql::{Mysql, parse_mysql_value};
#[cfg(feature = "feat-redis")]
use crate::config::redis::{parse_redis_value, Redis};
use crate::config::smtp::{parse_smtp_value, Smtp};
use crate::get_env;

pub mod app;
#[cfg(feature = "feat-mysql")]
pub mod mysql;
#[cfg(feature = "feat-redis")]
pub mod redis;
mod encrypt;
mod smtp;

pub static CONFIG: OnceCell<Config> = OnceCell::const_new();

/// # 初始化：读取配置文件
/// **Notice: 应用配置文件名称约定 `app-[环境].yaml` 或者 app-[环境].json 格式，例如：app-dev.yml。**
pub fn init_read_config() {
    let file_content = match get_file_content() {
        Ok(content) => content,
        Err(err) => {
            tracing::error!("读取配置文件时发生异常, {}", err);
            process::exit(0);
        }
    };

    let value = match get_value(&file_content) {
        Ok(value) => value,
        Err(err) => {
            tracing::error!("读取配置文件时发生异常, {}", err);
            process::exit(0);
        }
    };

    let config = parse_value(&value);
    CONFIG.set(config).unwrap();
}

fn get_file_content() -> Result<FileContent, Box<dyn Error>> {
    let env = match get_env() {
        Some(env) => env,
        None => {
            tracing::error!("未初始化环境变量。");
            process::exit(0);
        }
    };

    let active_profile = match env.clone().active_profile {
        Some(profile) => profile,
        None => "dev".to_string()
    };

    let files = vec![
        format!("./app-{}.yml", active_profile),
        format!("./app-{}.yaml", active_profile),
        format!("./app-{}.json", active_profile),
    ];

    let mut content: FileContent = FileContent::new_none();
    let mut count = 0;

    for file in &files {
        if fs::metadata(file).is_ok() {
            if count > 0 {
                tracing::error!("同一个环境的配置文件只能存在一个。");
                process::exit(0);
            }
            content = FileContent::from(read_file(file).unwrap(), file);
            count += 1;
        }
    }

    if count == 0 {
        tracing::error!("缺少配置文件。");
        process::exit(0);
    }

    Ok(content)
}

fn read_file(file_path: &str) -> Result<String, Box<dyn Error>> {
    fs::read_to_string(file_path)
        .map_err(|e| format!("读取文件({})时发生错误: {}", file_path, e).into())
}

fn get_value(file_content: &FileContent) -> Result<Value, Box<dyn Error>> {
    let configure = match file_content {
        FileContent::Json(content) => serde_json::from_str::<Value>(&content)
            .map_err(|e| { format!("读取配置文件时发生错误: {}", e) })?,
        FileContent::Yaml(content) => serde_yaml::from_str::<Value>(&content)
            .map_err(|e| { format!("读取配置文件时发生错误: {}", e) })?,
        FileContent::Yml(content) => serde_yaml::from_str::<Value>(&content)
            .map_err(|e| { format!("读取配置文件时发生错误: {}", e) })?,
        FileContent::None => return Err("配置文件不存在。".into()),
    };

    Ok(configure)
}

fn parse_value(value: &Value) -> Config {
    let app = parse_app_value(value);
    #[cfg(feature = "feat-mysql")]
    let mysql = parse_mysql_value(value);
    #[cfg(feature = "feat-redis")]
    let redis = parse_redis_value(value);
    let encrypt = parse_encrypt_value(value);
    #[cfg(feature = "feat-smtp")]
    let smtp = parse_smtp_value(value);

    let config = Config {
        app,
        #[cfg(feature = "feat-mysql")]
        mysql,
        #[cfg(feature = "feat-redis")]
        redis,
        encrypt,
        smtp
    };

    config
}

enum FileContent {
    Json(String),
    Yaml(String),
    Yml(String),
    None,
}

impl FileContent {
    fn new_none() -> Self {
        FileContent::None
    }

    fn from(content: String, file: &str) -> Self {
        let file_extension = get_file_extension(file);
        match file_extension {
            Some("json") => FileContent::Json(content),
            Some("yaml") => FileContent::Yaml(content),
            Some("yml") => FileContent::Yml(content),
            _ => FileContent::Json(content)
        }
    }
}

fn get_file_extension(file: &str) -> Option<&str> {
    Path::new(file)
        .extension()
        .and_then(|ext| ext.to_str())
}

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub app: Option<App>,
    #[cfg(feature = "feat-mysql")]
    pub mysql: Option<Mysql>,
    #[cfg(feature = "feat-redis")]
    pub redis: Option<Redis>,
    pub encrypt: Option<Encrypt>,
    #[cfg(feature = "feat-smtp")]
    pub smtp: Option<Smtp>,
}

