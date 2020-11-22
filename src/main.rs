#[macro_use]
extern crate lazy_static;
use actix_web::{App, HttpServer};
use fast_log::log::RuntimeType;
use rbatis::rbatis::Rbatis;

mod controller;
mod entity;
extern crate chrono;

//示例 mysql 链接地址
pub const MYSQL_URL: &'static str = "mysql://root:123456@localhost:3306/simple";

// 示例-Rbatis示例初始化(必须)
lazy_static! {
    static ref RB: Rbatis = Rbatis::new();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    RB.link(MYSQL_URL).await.unwrap();
    //启用日志输出，你也可以使用其他日志框架，这个不限定的
    fast_log::log::init_log("requests.log", &RuntimeType::Std).unwrap();

    HttpServer::new(|| App::new().configure(controller::user_controller::init))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
