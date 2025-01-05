use actix_web::{dev::HttpServiceFactory, get, post, web, HttpResponse};
use global::DB;
use serde::{Deserialize, Serialize};
use serverx::httpx;

// 设置路由
pub fn config() -> impl HttpServiceFactory{
    web::scope("/user").wrap(middleware::example::SayHi)
    .service(hello)
    // .service(add_user)
    .service(add_user1)
}

#[get("/hello")]
pub async fn hello(data: web::Data<DB>) -> String {

    let m = &data.mysql;
    
    m.to_string()
}

#[derive(Deserialize,Serialize)]
struct UserReq {
    username: String,
    age : u32
}

#[derive(Serialize, Deserialize)]
struct Info<'a> {
    info: &'a str
}

// #[post("/add_user")]
// pub async fn add_user<'a>(formdata: web::Json<UserReq>) -> httpx::ResponseJson<Info<>> {

//     println!("{},{}",formdata.username,formdata.age);

//     // serverx::ResponseJson{
//     //     status:1,
//     //     msg:"".to_string(),
//     //     data: Info { info: "info ".to_string() }
//     // }
//     // httpx::response(Info { info: "sdasa" });
//     // serverx::response_er(302, "这是一个错误")
    
//     httpx::response(Info { info: "hello world" })
//     // httpx::response_err(302, "hello")
// }

#[post("/add_user1")]
pub async fn add_user1(_obj: web::Json<UserReq>) -> Result<HttpResponse,httpx::ApiError> {
    // Ok(HttpResponse::Ok().json(UserReq{username:"cjp".to_string(),age:28}))
    
    // let u = UserReq{username:"cjp".to_string(),age:28};

    // Ok(httpx::response_success(u));
    
    Err(httpx::response_err(301, "用户登录错误"))
}