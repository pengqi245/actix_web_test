use crate::entity::test_entity::MyObj;
use crate::entity::user_entity::UserEntity;
use crate::RB;
use actix_web::{delete, get, post, put, web, HttpResponse};
use chrono::prelude::*;
use serde_json::json;

#[get("/test")]
pub async fn test() -> HttpResponse {
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
async fn add_user(user: web::Json<UserEntity>) -> HttpResponse {
    let create_date = match user.cteate_date {
        None => {
            let local: DateTime<Local> = Local::now();
            local.naive_local()
        }
        Some(i) => i,
    };
    let sql = format!(
        r#" 
        insert into sys_user (name, email, phone_number, create_date) value ("{}","{}","{}","{}")
        "#,
        user.name, user.email, user.phone_number, create_date
    );
    RB.py_exec("", &sql, &json!({})).await.unwrap();
    println!("{:?}", user);
    HttpResponse::Ok().body("operation successful")
}

#[delete("/user/{id}")]
async fn delete_user(web::Path(id): web::Path<i32>) -> HttpResponse {
    let sql = format!(
        r#" 
        delete from sys_user where id = {}
        "#,
        id
    );
    RB.py_exec("", &sql, &json!({})).await.unwrap();
    HttpResponse::Ok().body("operation successful")
}

#[put("/user")]
async fn update_user(user: web::Json<UserEntity>) -> HttpResponse {
    let id = match user.id {
        None => return HttpResponse::BadRequest().body("Id cannot empty!"),
        Some(i) => i,
    };
    let sql = format!(
        r#" 
        update sys_user set name="{}",email="{}",phone_number={} where id = {}
        "#,
        user.name, user.email, user.phone_number, id
    );
    RB.py_exec("", &sql, &json!({})).await.unwrap();
    HttpResponse::Ok().body("operation successful")
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(test);
    cfg.service(get_users);
    cfg.service(add_user);
    cfg.service(delete_user);
    cfg.service(update_user);
}
