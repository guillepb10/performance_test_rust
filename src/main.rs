use hyper::{Body, Request, Response, Server};
use hyper::rt::Future;
use hyper::service::service_fn;
use hyper::{Method, StatusCode};

use rand;
use rand::{RngCore};

use futures::future;

use regex::Regex;

use serde::Serialize;

type BoxFut = Box<dyn Future<Item=Response<Body>, Error=hyper::Error> + Send>;

#[derive(Serialize)]
struct ValueList {
    list: Vec<u32>
}

impl ValueList {
    fn new() -> ValueList {
        ValueList { list: Vec::new() }
    }
    fn add(&mut self, n: u32) {
        self.list.push(n);
    }
}


fn main() {
    let addr = ([127, 0, 0, 1], 8080).into();

    let server = Server::bind(&addr)
                    .serve(|| {
                        service_fn(route_rand_number)
                    })
                    .map_err(|e| eprintln!("server error: {}", e));

    hyper::rt::run(server);

}

fn route_rand_number(req: Request<Body>) -> BoxFut {
    let mut response = Response::new(Body::empty());
    let path = Regex::new("^/random/([0-9]*)$").unwrap();
    if req.method() == Method::GET && path.is_match(req.uri().path()) {
        response = rand_numbers(path.captures(req.uri().path()).unwrap().get(1).map(|n| n.as_str().parse::<u16>().unwrap()).unwrap())
    } else {
        *response.status_mut() = StatusCode::NOT_FOUND;
    }
    Box::new(future::ok(response))
}

fn rand_numbers(n: u16) -> Response<Body> {
    let mut rng = rand::thread_rng();
    let mut list = ValueList::new();
    for _ in 0..n {
        list.add(rng.next_u32());
    }
    Response::new(Body::from(serde_json::to_string(&list).unwrap()))
}