use std::fmt::Display;

use serde::{Serialize};
use actix_web::{body::BoxBody, HttpResponse, HttpResponseBuilder, ResponseError};

// #[derive(Serialize, Deserialize)]
// pub struct ResponseJson<T>{
//     pub status:u32,
//     pub msg: &'static str,
//     pub data:Option<T>
// }

// impl<'a,T:Serialize> Responder for ResponseJson<T> {
//     type Body = BoxBody;

//     fn respond_to(self, _req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
//         let body = serde_json::to_string(&self).unwrap();

//         // Create response and set content type
//         HttpResponse::Ok()
//             .content_type(ContentType::json())
//             .body(body)
//     }
// }

// // 正常返回
// pub fn response<'a,T>(data:T) -> ResponseJson<T>{
//     ResponseJson{
//         status:1,
//         msg:"success",
//         data:Some(data)
//     }
// }

#[derive(Debug)]
pub struct ApiError{
    status:u32,
    msg: &'static str
}

impl Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{{\"status\":{}, \"msg\":\"{}\"}}",self.status,self.msg)
    }
}

impl ApiError {
    pub fn new(status:u32,content:&'static str)->ApiError{
        ApiError {
            status:status, 
            msg: &content
        }
    }
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponseBuilder::new(self.status_code())
            .append_header(("content-type","application/json"))
            .body(self.to_string())
    }

    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::OK
    }
}

impl From<actix_web::error::Error> for ApiError {
    fn from(_error: actix_web::error::Error) -> Self {
        // 自定义转换逻辑
        ApiError::new(200, "")
    }
}


// 成功
pub fn response_success<T>(data:T)->HttpResponse
    where 
    T:Serialize
{
    
    HttpResponse::Ok().json(data)
}

// 报错
pub fn response_err(code:u32,msg:&'static str)->ApiError {

   ApiError::new(code, msg)
}