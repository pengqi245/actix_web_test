#[macro_use]
extern crate lazy_static;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use fast_log::log::RuntimeType;
use rbatis::rbatis::Rbatis;

mod endpoints;


//示例 mysql 链接地址
pub const MYSQL_URL: &'static str = "mysql://root:123456@localhost:3306/user";

// 示例-Rbatis示例初始化(必须)
lazy_static! {
    static ref RB:Rbatis=Rbatis::new();
}

// default / handler
async fn index() -> impl Responder {
    HttpResponse::Ok().body(r#"
        Welcome to Actix-web with SQLx Todos example.
        Available routes:
        GET /todos -> list of all todos
        POST /todo -> create new todo, example: { "description": "learn actix and sqlx", "done": false }
        GET /todo/{id} -> show one todo with requested id
        PUT /todo/{id} -> update todo with requested id, example: { "description": "learn actix and sqlx", "done": true }
        DELETE /todo/{id} -> delete todo with requested id
    "#
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    RB.link(MYSQL_URL).await.unwrap();
    //启用日志输出，你也可以使用其他日志框架，这个不限定的
    fast_log::log::init_log("requests.log", &RuntimeType::Std).unwrap();

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .configure(endpoints::routes::init)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}