use actix_web::{web, App, HttpServer};
use sea_orm::{Database, DatabaseConnection};

pub fn start_server(host: &Option<String>, port: &Option<String>) -> std::io::Result<()> {
    let conf = config::load_config().unwrap();

    let host = host.clone().unwrap_or_else(|| conf.http.host.to_string());
    let port = port.clone().unwrap_or_else(|| conf.http.port.to_string());

    let ipaddress = format!("{}:{}", host, port);
    println!("Starting HTTP server on {}", ipaddress);

    let mysql_url = format!("mysql://{}:{}@{}:{}/{}",
                            conf.mysql.username,
                            conf.mysql.password,
                            conf.mysql.host,
                            conf.mysql.port,
                            conf.mysql.database
    );

    // 使用 actix_rt 来运行异步代码
    actix_rt::System::new().block_on(async {

        let db: DatabaseConnection = Database::connect(mysql_url).await.unwrap();

        let g = global::DB{mysql:db};
        let gdb= web::Data::new(g);

        HttpServer::new(move || {
            App::new().app_data(gdb.clone())
                .configure(controller::api::config)
        })
            .bind(ipaddress)?
            .run()
            .await
    })?;

    Ok(())
}