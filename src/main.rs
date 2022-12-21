mod client;
mod messages;
mod recipe;
mod server;

use client::*;
use server::*;

use std::thread;

/// Main function
#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let thread1 = thread::spawn(|| {
        let john = Client::new();
        let server = Server { client: john };
        server.start();
    });
    let thread2 = thread::spawn(|| {
        let mark = Client::new();
        let server = Server { client: mark };
        server.start();
    });
    let thread3 = thread::spawn(|| {
        let bruce = Client::new();
        let server = Server { client: bruce };
        server.start();
    });
    thread1.join().unwrap();
    thread2.join().unwrap();
    thread3.join().unwrap();
}
