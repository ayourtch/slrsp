use std::collections::HashMap;

extern crate mustache;
extern crate router;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate serde_frommap;

#[macro_use]
extern crate rspten;
#[macro_use]
extern crate rspten_derive;

use rspten::RspState;
use rspten::*;

mod pages;

fn main() {
    use router::Router;

    let mut router = Router::new();
    router.get("/", pages::teststate::PageState::handler, "/");
    router.post("/", pages::teststate::PageState::handler, "/");
    router.get("/x", pages::test2state::PageState::handler, "/x");

    let mut s = rspten::RspServer::new();

    s.run(router, "test service", 4480);
}
