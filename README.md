# Rust-Boot

## Quick start

1. Cargo.toml 中添加依赖
    ```toml
    [dependencies]
    rust-boot = { git = "https://github.com/hi-liyan/rust-boot", features = ["feat-mysql", "feat-redis"] }
    ```
   
2. 项目根目录下添加配置文件：
   
   配置文件命名约定：`app-[active_profile].yml`，例如 `app-dev.yml`。配置文件支持 yaml 和 json 两种格式。 
   ```yaml
   app:
     name: example
     version: 1.0.0
     port: 8080
   
   mysql:
     host: localhost
     port: 3306
     user: root
     pass: 123456
     db_name: example_db
     max_connections: 20
   
   redis:
     host: localhost
     port: 6379
     pass: 123456
   
   encrypt:
     key: Te7CIEJOrKpWQ161
   
   smtp:
     host: smtp.example.com
     port: 465
     user: example@example.com
     pass: 123456
     max_size: 10 # 连接池的最大连接数，默认值 10
     min_idle: 0 # 最小空闲连接数，默认值 0
     idle_timeout: 60 # 空闲超时时间（单位：秒），默认值 60 
   ```

3. main() 函数中启动服务
    ```rust
    #[tokio::main]
    async fn main() {
        AppStart::new()
            .router(routers())
            .start()
            .await;
    }
    ```

## Environment variable list

1. `active_profile` 启用的配置文件，默认值：dev
2. `lov_level` 日志等级，默认值：debug

## features
1. `feat-mysql` 开启 MySQL 特性
2. `feat-redis` 开启 Redis 特性
3. `feat-smtp` 开启 SMTP 特性，配置 SMTP 发送邮件

## API
1. `get_env` 获取环境变量。
2. `get_config` 获取应用配置。
3. `get_mysql` 获取 MySQL 实例。（在启用 mysql feature 时可用）
4. `get_redis` 获取 Redis 实例。（在启用 redis feature 时可用）
5. `get_mailer` 获取 SMTP 实例。（在启用 smtp feature 时可用）
