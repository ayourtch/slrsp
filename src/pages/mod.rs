mod imports;
use rspten::RspState;
use rspten::*;

pub mod html_helpers;

mod test2state;
mod teststate;

pub fn get_router() -> router::Router {
    use router::Router;

    let mut router = Router::new();
    router.get("/", teststate::PageState::handler, "/");
    router.post("/", teststate::PageState::handler, "/");
    router.get("/x", test2state::PageState::handler, "/x");
    router
}
