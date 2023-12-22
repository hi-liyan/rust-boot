use std::process;
use std::time::Duration;

use lettre::{SmtpTransport};
use lettre::transport::smtp::authentication::Credentials;
use lettre::transport::smtp::PoolConfig;
use tokio::sync::OnceCell;

use crate::get_config;

#[cfg(feature = "feat-smtp")]
pub static MAILER: OnceCell<SmtpTransport> = OnceCell::const_new();

#[cfg(feature = "feat-smtp")]
pub fn init_mailer() {
    let config = match get_config() {
        Some(config) => config.clone(),
        None => {
            tracing::error!("无法获取配置，可能是配置文件不存在。");
            process::exit(0);
        }
    };

    let smtp_config = match config.smtp {
        Some(config) => config,
        None => {
            tracing::error!("配置文件中缺少 \"smtp\" 配置。");
            process::exit(0);
        }
    };

    let host = &smtp_config.host.unwrap_or_else(|| {
        tracing::error!("缺少 \"smtp.host\" 配置。");
        process::exit(0);
    });

    let port = &smtp_config.port.unwrap_or_else(|| {
        tracing::error!("缺少 \"smtp.port\" 配置。");
        process::exit(0);
    });

    let user = &smtp_config.user.unwrap_or_else(|| {
        tracing::error!("缺少 \"smtp.user\" 配置。");
        process::exit(0);
    });

    let pass = &smtp_config.pass.unwrap_or_else(|| {
        tracing::error!("缺少 \"smtp.pass\" 配置。");
        process::exit(0);
    });

    let max_size = &smtp_config.max_size.unwrap_or(10);
    let min_idle = &smtp_config.min_idle.unwrap_or(0);
    let idle_timeout = &smtp_config.idle_timeout.unwrap_or(60);

    let creds = Credentials::new(user.to_owned(), pass.to_owned());

    let pool_config = PoolConfig::new()
        .max_size(*max_size)
        .min_idle(*min_idle)
        .idle_timeout(Duration::from_secs(*idle_timeout));

    let mailer = SmtpTransport::relay(host)
        .unwrap()
        .credentials(creds)
        .port(*port)
        .pool_config(pool_config)
        .build();

    tracing::debug!("SMTP 初始化完成。");
    let _ = MAILER.set(mailer);
}