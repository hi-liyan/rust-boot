use time::macros::format_description;
use time::UtcOffset;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::fmt::time::OffsetTime;

use crate::get_env;

pub fn init_log() {
    let env = get_env().expect("获取环境变量是发生异常。");

    let local_time = OffsetTime::new(
        UtcOffset::from_hms(8, 0, 0).unwrap(),
        format_description!("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]"),
    );

    let log_level = match env.clone().log_level {
        Some(level) => level,
        None => "debug".to_string()
    };

    let subscriber = tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new(&log_level))
        .with_timer(local_time)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    tracing::debug!("日志初始化完成，日志级别：{}", &log_level);
}