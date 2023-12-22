use std::process;
use tokio::sync::OnceCell;
use sqlx::mysql::MySqlPoolOptions;
use sqlx::{MySql, Pool};
use crate::config::mysql::Mysql;
use crate::get_config;

pub static MYSQL: OnceCell<Pool<MySql>> = OnceCell::const_new();

/// 初始化：Mysql
pub async fn init_mysql() {
    let config = get_config().unwrap_or_else(|| {
        tracing::error!("无法获取配置，可能是配置文件不存在。");
        process::exit(0);
    });

    let mysql = match config.clone().mysql {
        Some(db) => db,
        None => {
            tracing::error!("配置文件中缺少 \"mysql\" 配置。");
            process::exit(0);
        }
    };

    let url = get_url(&mysql);
    tracing::debug!("MySQL url: {}", url);

    let pool = MySqlPoolOptions::new()
        .max_connections(mysql.max_connections.unwrap_or(30))
        .connect(&url)
        .await
        .unwrap_or_else(|e| {
            tracing::error!("发生错误：{:?}", e);
            process::exit(0);
        });

    MYSQL.set(pool).unwrap();
    tracing::debug!("MySQL 初始化完成。")
}

fn get_url(db: &Mysql) -> String {
    let user = db.user.clone().unwrap_or_else(|| {
        tracing::error!("缺少 \"mysql.host\" 配置。");
        process::exit(0);
    });

    let pass = db.pass.clone().unwrap_or_else(|| {
        tracing::error!("缺少 \"mysql.pass\" 配置。");
        process::exit(0);
    });

    let host = db.host.clone().unwrap_or_else(|| {
        tracing::error!("缺少 \"mysql.host\" 配置。");
        process::exit(0);
    });

    let port = db.port.clone().unwrap_or_else(|| {
        tracing::error!("缺少 \"mysql.port\" 配置。");
        process::exit(0);
    });

    let db_name = db.db_name.clone().unwrap_or_else(|| {
        tracing::error!("缺少 \"mysql.db_name\" 配置。");
        process::exit(0);
    });

    format!("mysql://{}:{}@{}:{}/{}?timezone=Asia/Shanghai&useSSL=false",
            user,
            pass,
            host,
            port,
            db_name
    )
}