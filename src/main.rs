extern crate rnginx;

use rnginx::server;

fn main() {
    let server = server::Server::new("baka", "127.0.0.1:23333");
    server.boot();
}
