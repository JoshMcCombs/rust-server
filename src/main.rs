fn main() {
    // let server = Server::new("127.0.0.1:8005");
    // Server.run();
    let string = String::from("127.0.0.1:8005");
    let str_slice = &string[13..]
}

struct Server {
    addr: String,
} 

impl Server {
    fn new(addr: String) -> Self {
        Server {
            addr: addr
        }
    }

    fn run(self) {

    }
}