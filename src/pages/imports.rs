pub use rspten::RspState;
pub use rspten::*;

pub use mustache::MapBuilder;
pub use mustache::Template;
pub use rspten::RspAction;
pub use rspten::RspEvent;

pub use std::collections::HashMap;

pub struct MyPageType {}
impl rspten::RspPageType for MyPageType {}
