pub use rspten::RspState;
pub use rspten::*;

pub use mustache::MapBuilder;
pub use mustache::Template;
pub use rspten::RspAction;
pub use rspten::RspEvent;

pub use std::collections::HashMap;

extern crate iron;

pub struct NoPageAuth {}
impl rspten::RspUserAuth for NoPageAuth {
    fn from_request(req: &mut iron::Request) -> NoPageAuth {
        NoPageAuth {}
    }
    fn auth_action(auth: &Self) -> RspAuthAction {
        rspten::RspAuthAction::Render
    }

    fn has_rights(auth: &Self, rights: &str) -> bool {
        false
    }
}
