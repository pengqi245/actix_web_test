use super::model::{MyObj, UserModel};
use crate::RB;
use actix_web::{get,post, web, HttpResponse};
use serde_json::{json};

#[get("/test")]
pub async fn test() -> HttpResponse{
    HttpResponse::Ok().json(MyObj {
        name: String::from("test"),
        number: String::from("123456"),
    })
}


#[get("/users")]
async fn get_users() -> HttpResponse {
    let sql = r#" 
        select * from sys_user
        "#;
    let data: serde_json::Value = RB.py_fetch("", sql, &json!({})).await.unwrap();
    println!("{}", data);
    HttpResponse::Ok().body(data)
}

#[post("/user")]
async fn add_user(user: web::Json<UserModel>) -> HttpResponse {

    println!("{:?}", user);
    // let sql = r#" 
    //     select * from user
    //     "#;
    // let data: Vec<User>= RB.py_fetch("", sql, &json!({})).await.unwrap();
    HttpResponse::Ok().body("hello word")
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(test);
    cfg.service(get_users);
    cfg.service(add_user);
}
