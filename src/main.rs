use rust_json_server::Server;

fn main() {
   let server = Server::new("127.0.0.1:7878");
   server.run();
}
