#![feature(macro_rules)]
#![feature(phase)]

#[phase(plugin)] extern crate docopt_macros;
extern crate docopt;
extern crate serialize;
extern crate nickel;
extern crate time;
extern crate http;
extern crate url;

use keymanager::KeyManager;
use docopt::FlagParser;
use nickel::{ Action, Continue, Nickel, NickelError, Middleware, Request, Response };
use worker::{Uuid, IdWorker};
use std::io::net::ip::Ipv4Addr;

mod worker;
mod keymanager;

#[deriving(Clone)]
struct IdWorkerMiddleware {
    worker: IdWorker
}

impl IdWorkerMiddleware {
    fn new (worker_id: i64, datacenter_id: i64) -> IdWorkerMiddleware {
        IdWorkerMiddleware { worker: IdWorker::new(worker_id, datacenter_id) }
    }
}

impl Middleware for IdWorkerMiddleware {
    fn invoke (&self, req: &mut Request,
               _: &mut Response) -> Result<Action, NickelError> {
        req.map.insert(self.worker.next());
        Ok(Continue)
    }
}

fn uuid_handler (_request: &Request, response: &mut Response) {
    let uuid = match _request.map.find::<Uuid>() {
        Some(id) => id.bytes.to_string(),
        None => fail!("could not retrieve uuid")
    };

    response.send(uuid);
}


docopt!(Args, "
Usage: rustedflake <etcd-quorum> <datacenter-id>
")
fn main() {
    let args: Args = FlagParser::parse().unwrap_or_else(|e| e.exit());
    println!("{}", args);
    
    let datacenter_id: i64 = from_str(args.arg_datacenter_id.as_slice()).unwrap();
    let key_manager = KeyManager::new(args.arg_etcd_quorum.as_slice());
    let worker_id = key_manager.get_next_worker_id();
    
    let mut server = Nickel::new();
    server.utilize(IdWorkerMiddleware::new(worker_id, datacenter_id));
    server.get("/uuid", uuid_handler);
    server.listen(Ipv4Addr(0, 0, 0, 0), 8080);
}
