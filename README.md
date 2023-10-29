# Rust-Boot

## Quick start

1. Cargo.toml 中添加依赖
    ```toml
    [dependencies]
    rust-boot = { git = "https://github.com/hi-liyan/rust-boot", features = ["mysql", "redis"] }
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
1. `mysql`
2. `redis`

## API
1. `get_env` 获取环境变量。
2. `get_config` 获取配置信息
3. `get_mysql` 获取 MySQL 实例。（在启用 mysql features 时可用）
4. `get_redis` 获取 Redis 实例。（在启用 redis features 时可用）
