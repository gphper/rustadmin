use actix_web::{web, App, HttpServer};
use sea_orm::*;
#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let conf = config::load_config().unwrap();
    let mysql_url = format!("mysql://{}:{}@{}:{}/{}",
                           conf.mysql.username,
                           conf.mysql.password,
                           conf.mysql.host,
                           conf.mysql.port,
                           conf.mysql.database
    );

    let db: DatabaseConnection = Database::connect(mysql_url).await.unwrap();

    let g = global::DB{mysql:db};
    let gdb= web::Data::new(g);

    HttpServer::new(move || {
        App::new().app_data(gdb.clone())
            .configure(controller::api::config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}