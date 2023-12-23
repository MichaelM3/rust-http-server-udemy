fn main() {
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}

struct Server {
    addr: String,
}

impl Server {
    fn new(addr: String) -> Self {
        Self { addr }
    }

    fn run(self) {
        println!("Listening on port: {}", self.addr)
    }
}

struct Request {
    path: String,
    query_string: String,
    method: String,
}

enum Method {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
}

/*
    GET /user?id=10 HTTP/1.1\r\n
    HEADERS \r\n
    BODY
*/
