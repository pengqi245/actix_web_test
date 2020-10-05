use super::model::MyObj;
use crate::RB;
use actix_web::{get, web, HttpResponse, Result};
use serde_json::{json, Value};

#[get("/test")]
pub async fn test() -> HttpResponse{
    HttpResponse::Ok().json(MyObj {
        name: String::from("test"),
        number: String::from("123456"),
    })
}

#[get("/test2")]
async fn test2() -> HttpResponse {
    let v: Result<i32, rbatis_core::Error> = RB.fetch("", "SELECT count(1) FROM user;").await;
    HttpResponse::Ok().body(format!("count(1)={}", v.unwrap_or(0)))
}

#[get("/test3")]
async fn test3() -> HttpResponse {
    let sql = r#" 
        select * from user
        "#;
    let data: serde_json::Value = RB.py_fetch("", sql, &json!({})).await.unwrap();
    println!("{}", data);
    HttpResponse::Ok().body(data)
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(test);
    cfg.service(test2);
    cfg.service(test3);
}
