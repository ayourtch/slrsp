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

// #[macro_use]
extern crate chrono;
extern crate iron;
extern crate iron_sessionstorage;

mod html_helpers;
mod pages;

fn main() {
    let router = pages::get_router();

    let mut s = rspten::RspServer::new();

    s.run(router, "test service", 4480);
}
