use actix_web::web::{self};

pub mod user;
pub mod article;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
        // 文章
        .service(article::config())
        // 用户
        .service(user::config())
    );
}