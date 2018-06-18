extern crate rnginx;

use rnginx::server;
use std::process;

fn main() {
    let server = server::Server::new("baka", "127.0.0.1:23333");
    if let Err(e) = server.boot() {
        eprintln!("boot server failure");
        process::exit(0x0100);
    }
}
