# rustadmin
Rust 开发脚手架

# 命令行
`cargo run -- 子命令`

# 日志
`tracing::info!(target:LogTarget::WEB_LOG,"web log message")`

# 数据库
## 创建迁移文件
`sea-orm-cli migrate generate -d dbs/src/mysqlx/migrator create_book
`
## 生成 entity 文件
`sea-orm-cli generate entity -t 数据表 -u 数据库连接 -o dbs/src/mysqlx/entities`