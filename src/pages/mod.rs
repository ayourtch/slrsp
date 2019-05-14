mod imports;
use rspten::RspState;
// use rspten::*;

// pub mod html_helpers;

make_get_router!(
    test2state    "/x",
    teststate     "/",
    open_teststate     "/open",
    login "/Administration/Login.aspx",
    logout "/logout",
);
