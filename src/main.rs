fn main() {
  let server = server::Server::new("127.0.0.1:8899".to_string());
  server.listen();
}

mod server {
  pub struct Server {
    address: String,
  }

  impl Server {
    pub fn new(address: String) -> Self {
      Self { address }
    }
    pub fn listen(self) {
      println!("Listening on port {}", self.address);
    }
  }
}
struct Request {
  path: String,
  query_string: Option<String>,
  method: Method,
}

enum Method {
  GET,
  POST,
  DELETE,
  PUT,
  HEAD,
  CONNECT,
  OPTIONS,
  TRACE,
  PATCH,
}
