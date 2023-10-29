use std::env;
use std::env::VarError;
use serde::Deserialize;
use tokio::sync::OnceCell;

use crate::common::env_const::{ACTIVE_PROFILE, LOG_LEVEL};

pub static ENV: OnceCell<Env> = OnceCell::const_new();

/// # 初始化：读取环境变量
/// ## 环境变量列表：
/// `active_profile` 激活的配置文件，默认app-dev.yml/app-dev.json
pub fn init_read_env() {
    let active_profile = match env::var(ACTIVE_PROFILE) {
        Ok(profile) => Some(profile),
        Err(_) => None
    };

    let log_level = match env::var(LOG_LEVEL) {
        Ok(level) => Some(level),
        Err(_) => None
    };

    let env = Env {
        active_profile,
        log_level
    };

    ENV.set(env).unwrap();
}

#[derive(Debug, Clone, Deserialize)]
pub struct Env {
    pub active_profile: Option<String>,
    pub log_level: Option<String>
}







