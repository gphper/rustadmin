use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = global::DB{mysql:"mysql conn...".to_string()};

    let gdb= web::Data::new(db);

    HttpServer::new(move || {
        App::new().app_data(gdb.clone())
            .configure(controller::api::config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}