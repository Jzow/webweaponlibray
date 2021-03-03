pub mod SingleController {
    // 加载模块
    use std::io::prelude::*;
    use std::net::TcpListener;
    use std::net::TcpStream;
    use std::fs;

    /*
     * 控制台console 输出信息
     * stream : 接收请求HTTP Stream流
     */
    pub fn hadle_connection(mut stream: TcpStream) {
        let mut buffer = [0; 1024];
        // 读取请求信息输出
        stream.read(&mut buffer).unwrap();
        println!("请求: {}", String::from_utf8_lossy(&buffer[..]));
        // 响应服务器
        let response = "HTTP/1.1 200 OK\r\n\r\n";
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }

    /*
     * 返回页面
     * stream : 接收请求HTTP Stream流
     */
    pub fn reponse_page(mut stream: TcpStream) {
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();

        let get = b"GET / HTTP/1.1\r\n";
        if buffer.starts_with(get) {
            let contents = fs::read_to_string("page/home.html").unwrap();
            // 响应拼装
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
                contents.len(),
                contents
            );
            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        } else {
            let status_line = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
            let contents = fs::read_to_string("page/404.html").unwrap();
            // 响应拼装
            let response = format!("{}{}", status_line, contents);
            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        }
    }

    /*
     * 重构返回页面
     * stream : 接收请求HTTP Stream流
     */
     pub fn reponse_struct_page(mut stream: TcpStream) {
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();
        let get = b"GET / HTTP/1.1\r\n";

        let (status, filename) = if buffer.starts_with(get) {
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