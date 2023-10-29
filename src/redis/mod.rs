use std::process;

use redis::{Client, RedisResult};
use tokio::sync::OnceCell;

use crate::get_config;

pub static REDIS_CLIENT: OnceCell<Client> = OnceCell::const_new();

pub fn init_redis() {
    let config = match get_config() {
        Some(config) => config.clone(),
        None => {
            tracing::error!("无法获取配置，可能是配置文件不存在。");
            process::exit(0);
        }
    };

    let redis_config = match config.redis {
        Some(config) => config,
        None => {
            tracing::error!("配置文件中缺少 \"redis\" 配置。");
            process::exit(0);
        }
    };

    let host = &redis_config.host.unwrap_or_else(|| {
        tracing::error!("缺少 \"redis.host\" 配置。");
        process::exit(0);
    });

    let pass = &redis_config.pass.unwrap_or("".to_string());

    let port = &redis_config.port.unwrap_or_else(|| {
        tracing::error!("缺少 \"redis.port\" 配置。");
        process::exit(0);
    });

    let url = format!("redis://:{}@{}:{}/", pass, host, port);
    tracing::debug!("Redis url: {}", url);

    let redis = match Client::open(url) {
        Ok(redis) => redis,
        Err(err) => {
            tracing::error!("Redis 连接异常：{:?}", err);
            return;
        }
    };
    tracing::debug!("Redis 初始化完成。");

    REDIS_CLIENT.set(redis).unwrap();
}