use tracing_subscriber::{prelude::*};
use tracing_log::LogTracer;
use tracing_subscriber::fmt::time::{FormatTime};
use tracing_subscriber::fmt::format::Writer;
use tracing_appender::non_blocking::WorkerGuard;

struct LocalTimer;

const fn east8() -> Option<chrono::FixedOffset> {
    chrono::FixedOffset::east_opt(8 * 3600)
}

impl FormatTime for LocalTimer {
    fn format_time(&self, w: &mut Writer<'_>) -> std::fmt::Result {
        let now = chrono::Utc::now().with_timezone(&east8().unwrap());
        write!(w, "{}", now.format("%Y-%m-%d %H:%M:%S"))
    }
}

pub struct LogTarget{}
impl LogTarget {
    pub const WEB_LOG:&'static str = "web-log";
    pub const API_LOG:&'static str = "api-log";
}

///
/// 日志配置初始化
///
pub fn init_log() -> Vec<WorkerGuard> {
    let mut guards = Vec::new();

    // 消费log门面日志 转为 tracing Event日志
    LogTracer::builder()
        // .with_max_level(log::LevelFilter::Error)
        .init()
        .expect("[PEAR] LogTracer 初始化失败");

    // 输出日志
    let console_subscriber = tracing_subscriber::fmt
    ::layer()
        .with_writer(std::io::stdout)
        .with_timer(LocalTimer)
        .with_filter(tracing_subscriber::filter::LevelFilter::INFO);

    // 特别定制web日志输出
    let file_appender = tracing_appender::rolling
    ::daily("logs/", LogTarget::WEB_LOG);
    // 生成非阻塞写入器
    let (non_blocking, guard1) = tracing_appender
    ::non_blocking(file_appender);
    guards.push(guard1);
    let file_subscriber = tracing_subscriber::fmt::layer().json()
        .with_writer(non_blocking)
        .with_ansi(false)
        .with_timer(LocalTimer)
        .with_filter(tracing_subscriber::filter::filter_fn(|metadata| {
            // Only enable spans or events with the target "interesting_things"
            metadata.target() == LogTarget::WEB_LOG
        }));

    // 特别定制api日志输出
    let file_appender_api = tracing_appender::rolling
    ::daily("logs/", LogTarget::API_LOG);
    // 生成非阻塞写入器
    let (non_blocking_api, guard) = tracing_appender
    ::non_blocking(file_appender_api);
    guards.push(guard);
    let file_subscriber_api = tracing_subscriber::fmt::layer().json()
        .with_writer(non_blocking_api)
        .with_ansi(false)
        .with_timer(LocalTimer)
        .with_filter(tracing_subscriber::filter::filter_fn(|metadata| {
            // Only enable spans or events with the target "interesting_things"
            metadata.target() == LogTarget::API_LOG
        }));

    // 集合
    let subscriber = tracing_subscriber::registry()
        .with(console_subscriber)
        .with(file_subscriber)
        .with(file_subscriber_api);

    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    guards
}