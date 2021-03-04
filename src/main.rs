mod ManyThreadWeb;
mod SingleThreadWeb;

use std::thread;
use std::time::Duration;
use std::net::TcpListener;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

use crate::SingleThreadWeb::SingleController;
use crate::ManyThreadWeb::ManyController;

struct AppState{
    app_name : String
}

/*
 * main 方法
 */
/*fn main() {
    // 绑定IP:端口 创建HTTP通信
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming(){
        let stream = stream.unwrap();
        println!("请求开始");
        // 单线程web服务器
        // SingleController::reponse_struct_page(stream);
        // 多线程web服务器
        thread::spawn(||{
            ManyController::hadle_conection(stream)
        });
    }
}*/
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(attr)
            .data(AppState{app_name: String::from("Actix-Web")}).service(index)
            .route("/hey", web::get().to(manual_hello))
    })
        .bind("127.0.0.1:8082")?
        .run()
        .await
}

#[get("/")]
async fn index(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name;
    format!("Hello {}!", app_name)
}

#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World")
}

#[post("/attr")]
async fn attr(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there")
}

