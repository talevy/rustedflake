#![feature(phase)]

extern crate nickel;
extern crate rustedflake;

#[phase(plugin)]
extern crate lazy_static;

use nickel::{ Nickel, Request, Response };
use rustedflake::IdWorker;
use std::io::net::ip::Ipv4Addr;
use std::sync::Arc;

lazy_static! {
    static ref ID_WORKER: Arc<IdWorker> = Arc::new(IdWorker::new(0, 0));
}

fn uuid_handler (_request: &Request, response: &mut Response) {
    let mut local_worker = ID_WORKER.clone();

    response.send(local_worker.make_unique().next().bytes.to_string());
}

fn main() {
    let mut server = Nickel::new();
    server.get("/uuid", uuid_handler);
    server.listen(Ipv4Addr(0, 0, 0, 0), 8080);
}
