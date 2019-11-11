use rand;
use rand::RngCore;

#[macro_use]
extern crate tower_web;
extern crate serde_json;

use tokio::prelude::future::{lazy, Future};
use tower_web::ServiceBuilder;

#[derive(Debug, Response)]
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

struct RandomServide;

impl_web! {
    impl RandomServide {

        #[get("/random/:size")]
        #[content_type("application/json")]
        fn get_random(&self, size: u32) -> impl Future<Item=ValueList, Error=()> {
            lazy(move || { Ok(rand_numbers(size)) })
        }

        #[get("/_health")]
        fn health(&self) -> Result<String, ()> {
            Ok(String::from("OK"))
        }
    }
}

fn main() {
    let addr = ([127, 0, 0, 1], 8080).into();
    println!("Listening on http://{}", addr);

    ServiceBuilder::new()
        .resource(RandomServide)
        .run(&addr)
        .unwrap();
}

fn rand_numbers(n: u32) -> ValueList {
    let mut rng = rand::thread_rng();
    let mut list = ValueList::new();
    for _ in 0..n {
        list.add(rng.next_u32());
    }
    list
}
