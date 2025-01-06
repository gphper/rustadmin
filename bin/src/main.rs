use clap::{Parser, Subcommand};
use bin::httpserver;
use bin::migrate::migrate;

fn main() {
    let cli = Cli::parse();

    match &cli.command {

        Commands::Migrate { tables } => {
            // 迁移数据表
            migrate(tables);
        }

        Commands::Server { host, port } => {
            // 启动服务
            httpserver::start_server(&host, &port).unwrap();
        }
    }
}

#[derive(Subcommand)]
enum Commands {
    /// migrate tables
    Migrate {
        /// database tables
        tables: Vec<String>
    },
    /// start server
    Server {
        /// host
        #[arg(long)]
        host:Option<String>,
        /// port
        #[arg(long)]
        port:Option<String>
    },
}


#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}