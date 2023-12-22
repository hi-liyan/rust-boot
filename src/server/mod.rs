use std::net::SocketAddr;
use std::process;
use axum::Router;

use crate::get_config;

/// # 初始化：启动 axum 服务
/// ## Params:
/// `router`: 路由
pub async fn init_server(router: Router) {
    let config = match get_config() {
        Some(config) => config.clone(),
        None => {
            tracing::error!("无法获取配置，可能是配置文件不存在。");
            process::exit(0);
        }
    };
    let app_config = match config.app {
        Some(app) => app,
        None => {
            tracing::error!("配置文件中缺少 \"app\" 配置。");
            process::exit(0);
        }
    };
    let port = match app_config.port {
        None => {
            tracing::debug!("没有指定端口号，启用默认端口 8080");
            8080
        }
        Some(port) => port
    };

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::debug!("监听端口：{}", port);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap_or_else(|e| {
        tracing::error!("发生错误：{}", e);
        process::exit(0);
    });

    axum::serve(listener, router.into_make_service()).await.unwrap_or_else(|e| {
        tracing::error!("发生错误：{}", e);
        process::exit(0);
    });
}