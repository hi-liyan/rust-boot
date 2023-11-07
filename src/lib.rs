#![feature(unboxed_closures)]

use std::process;

use ::redis::{Connection, RedisError};
use axum::Router;
use sqlx::{MySql, Pool};

use crate::config::{CONFIG, Config, init_read_config};
#[cfg(feature = "mysql")]
use crate::db::mysql::{init_mysql, MYSQL};
use crate::env::{Env, ENV, init_read_env};
use crate::log::init_log;
#[cfg(feature = "redis")]
use crate::redis::{init_redis, REDIS_CLIENT};
use crate::server::init_server;

mod env;
mod config;
mod common;
#[cfg(feature = "mysql")]
mod db;
mod log;
mod server;
#[cfg(feature = "redis")]
mod redis;

pub struct AppStart {
    router: Option<Router>,
}

impl AppStart {
    pub fn new() -> AppStart {
        init_read_env();        // step 1. 读取环境变量
        init_log();             // step 2. 初始化日志
        init_read_config();     // step 3. 读取应用配置文件

        AppStart {
            router: None
        }
    }

    pub fn router(self: &AppStart, router: Router) -> Self {
        AppStart {
            router: Some(router)
        }
    }

    pub async fn start(self: &AppStart) {
        check_router(self);

        #[cfg(feature = "mysql")]
        init_mysql().await;        // step 4. 初始化 Mysql
        #[cfg(feature = "redis")]
        init_redis();           // step 5. 初始化 Redis
        init_server(self.router.clone().unwrap()).await;    // step 6. 初始化axum服务
    }
}

fn check_router(app_start: &AppStart) {
    if let None = app_start.router {
        tracing::error!("调用 \"start()\" 函数前，需要先调用 \"router()\" 函数。");
        process::exit(0);
    }
}

/// # 获取环境变量配置
pub fn get_env() -> Option<&'static Env> {
    ENV.get()
}

/// # 获取应用配置
pub fn get_config() -> Option<&'static Config> {
    CONFIG.get()
}

/// # 获取数据库实例
#[cfg(feature = "mysql")]
pub fn get_mysql() -> &'static Pool<MySql> {
    MYSQL.get().unwrap()
}

/// # 获取Redis实例
#[cfg(feature = "redis")]
pub fn get_redis() -> Result<Connection, RedisError> {
    let client = REDIS_CLIENT.get().unwrap();
    client.get_connection()
}