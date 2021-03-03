pub mod ManyController {

    // 加载模块
    use std::io::prelude::*;
    use std::net::TcpListener;
    use std::net::TcpStream;
    use std::fs;
    use std::thread;
    use std::time::Duration;

    pub fn hadle_conection(mut stream : TcpStream){
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();

        let get = b"GET / HTTP/1.1\r\n";
        let sleep  = b"GET /sleep HTTP/1.1\r\n";

        let (status, filename) = if buffer.starts_with(get){
            ("HTTP/1.1 200 OK\r\n\r\n", "page/home.html")
        } else if buffer.starts_with(sleep) {
            thread::sleep(Duration::from_micros(5));
            println!("连接sleep");
            ("HTTP/1.1 200 OK\r\n\r\n", "page/home.html")
        } else {
            ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "page/404.html")
        };
        let contents = fs::read_to_string(filename).unwrap();
        let response = format!("{}{}", status, contents);
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}