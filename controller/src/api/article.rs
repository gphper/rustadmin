use actix_web::{dev::HttpServiceFactory, get, web};
use global::DB;

// 设置路由
pub fn config() -> impl HttpServiceFactory{
    web::scope("/article")
    .service(hello)
}

#[get("/hello")]
pub async fn hello(data: web::Data<DB>) -> String {

    let m = &data.mysql;
    
    m.to_string()
}