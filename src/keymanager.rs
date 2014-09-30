extern crate http;
extern crate url;
extern crate serialize;

use http::client::RequestWriter;
use http::method::{Get, Put};
use http::headers::HeaderEnum;
use url::Url;
use serialize::{json, Decodable};
use serialize::json::{Json, DecodeResult, Decoder, DecoderError};
use std::str;
use std::cmp::max;

static endpoint: &'static str = "/v2/keys/uuid_workers/";

#[deriving(Encodable, Decodable, Show)]
struct Node {
    key: String,
    value: String,
    createdIndex: int,
    modifiedIndex: int
}

#[deriving(Encodable, Decodable, Show)]
struct DirNode {
    key: String,
    dir: bool,
    nodes: Vec<Node>
}

#[deriving(Encodable, Decodable, Show)]
struct DirListing {
    action: String,
    node: DirNode
}

#[deriving(Encodable, Decodable, Show)]
pub struct KeyManager {
    etcd: String
}

impl KeyManager {
    pub fn new (etcd: &str) -> KeyManager {
        KeyManager { etcd: String::from_str(etcd) }
    }

    fn do_get<T: Decodable<Decoder, DecoderError>>(url: &str) -> DecodeResult<T> {
        let url = Url::parse(url).ok().expect("Invalid URL :-(");
        let request: RequestWriter = RequestWriter::new(Get, url).unwrap();
        let mut response = match request.read_response() {
            Ok(response) => response,
            Err(_request) => fail!("This example can progress no further with no response :-(")
        };

        let body = match response.read_to_end() {
            Ok(body) => body,
            Err(err) => fail!("Reading response failed: {}", err)
        };

        let bod = str::from_utf8(body.as_slice()).expect("uh oh");
        let json_bod = json::from_str(bod).unwrap();
        let mut decoder = json::Decoder::new(json_bod);

        return Decodable::decode(&mut decoder);
    }

    fn do_put(url: &str) {
        let url = Url::parse(url).ok().expect("Invalid URL :-(");
        let mut request: RequestWriter = RequestWriter::new(Put, url).unwrap();
        let data = b"value=hello";
        request.headers.content_length = Some(data.len());
        request.write(data);
        println!("ok...");
        let response = match request.read_response() {
                Ok(response) => response,
                Err((_request, error)) => fail!(":-( {}", error),
        };
    }

    pub fn get_next_worker_id (self) -> i64 {
        let url_str = self.etcd.append(endpoint);
        let res: DecodeResult<DirListing> = KeyManager::do_get(url_str.as_slice());
        match res {
            Ok(result) => {
                let mut max_id: i64 = 0;
                for node in result.node.nodes.iter() {
                    let num: i64 = from_str(node.key.as_slice().split('/').last().unwrap()).unwrap();
                    max_id = max(max_id, num);
                }

                let next_id = max_id + 1;

                println!("hello: {}", next_id);
                KeyManager::do_put(url_str.append(next_id.to_string().as_slice()).as_slice());

                next_id
            },
            Err(err) => {
                fail!("fail: {}", err);
                KeyManager::do_put(url_str.append("0").as_slice());
                0i64
            }
        }
    }
}





