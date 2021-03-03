mod ManyThreadWeb;
mod SingleThreadWeb;

use std::thread;
use std::time::Duration;
use std::net::TcpListener;


use crate::SingleThreadWeb::SingleController;
use crate::ManyThreadWeb::ManyController;


/*
 * main 方法
 */
fn main() {
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
}
