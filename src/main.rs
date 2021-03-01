use http::Method;
use http::Request;

mod server;
mod http;

fn main() {
  let port = "8899";
  let server = server::Server::new(format!("127.0.0.1:{}", port));
  server.listen();
}
