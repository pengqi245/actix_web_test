use super::model::{MyObj, User};
use crate::RB;
use actix_web::{get, web, HttpResponse};
use serde_json::{json};

#[get("/test")]
pub async fn test() -> HttpResponse{
    HttpResponse::Ok().json(MyObj {
        name: String::from("test"),
        number: String::from("123456"),
    })
}

#[get("/")]
async fn test2() -> HttpResponse {
    HttpResponse::Ok().body("hello word")
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

#[get("/test4")]
async fn test4() -> HttpResponse {
    let sql = r#" 
        select * from user
        "#;
    let data: Vec<User>= RB.py_fetch("", sql, &json!({})).await.unwrap();
    HttpResponse::Ok().json(data)
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(test);
    cfg.service(test2);
    cfg.service(test3);
    cfg.service(test4);
}
