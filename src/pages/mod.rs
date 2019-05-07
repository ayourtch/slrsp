mod imports;
use rspten::RspState;
use rspten::*;

// pub mod html_helpers;

make_get_router!(
    test2state    "/x",
    teststate     "/",
);
