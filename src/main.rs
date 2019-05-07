use std::collections::HashMap;

extern crate mustache;
extern crate router;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_frommap;
extern crate serde_json;

#[macro_use]
extern crate rspten;
#[macro_use]
extern crate rspten_derive;

use rspten::RspState;
use rspten::*;

mod pages;
mod html_helpers;

fn main() {
    let mut router = pages::get_router();

    let mut s = rspten::RspServer::new();

    s.run(router, "test service", 4480);
}
