#![allow(non_snake_case)]
use super::imports::*;

#[derive(RspTraits, Debug, Clone, Serialize, Deserialize)]
pub struct PageState {
    txtUsername: String,
    txtPassword: String,
    message: Option<String>,
    return_url: String,
}

type MyPageAuth = NoPageAuth;

impl RspState<String, MyPageAuth> for PageState {
    fn get_key(
        _auth: &MyPageAuth,
        args: &HashMap<String, Vec<String>>,
        maybe_state: &Option<PageState>,
    ) -> String {
        if let Some(st) = maybe_state {
            st.return_url.clone()
        } else {
            let root = vec!["/".to_string()];
            args.get("ReturnUrl").unwrap_or(&root)[0].clone()
        }
    }
    fn get_state(_auth: &MyPageAuth, key: String) -> PageState {
        PageState {
            txtUsername: "".to_string(),
            txtPassword: "".to_string(),
            message: None,
            return_url: key,
        }
    }
    fn fill_data(
        _auth: &MyPageAuth,
        data: MapBuilder,
        _ev: &RspEvent,
        _curr_key: &String,
        state: &mut Self,
        _initial_state: &Self,
        curr_initial_state: &Self,
    ) -> MapBuilder {
        let mut modified = false;
        let gd = || data;
        html_text!(gd, txtUsername, state, curr_initial_state, modified);
        html_text!(gd, txtPassword, state, curr_initial_state, modified);
        gd()
    }

    fn event_handler(
        req: &mut Request,
        _auth: &MyPageAuth,
        _ev: &RspEvent,
        _curr_key: &String,
        _maybe_state: &mut Option<PageState>,
        _maybe_initial_state: &Option<PageState>,
        _curr_initial_state: &PageState,
    ) -> RspAction<String> {
        let mut ret = rspten::RspAction::Render;
        if _ev.event == "submit" {
            if let Some(state) = _maybe_state {
                println!("Submit on login page");
                if &state.txtUsername == "user" && &state.txtPassword == "pass" {
                    let mut groups: HashMap<String, bool> = HashMap::new();
                    let username = state.txtUsername.clone();
                    println!("Success!");
                    let res = req
                        .session()
                        .set(CookiePageAuth::new(&username, Some(groups)));
                    match res {
                        Ok(x) => {
                            println!("OK: {:?}", &x);
                            ret = rspten::RspAction::RedirectTo(state.return_url.clone());
                        }
                        Err(e) => {
                            state.message = Some(format!("Error: {:?}", &e));
                        }
                    }
                } else {
                    println!("Login failure");
                    state.message = Some(format!("Login {} invalid", &state.txtUsername));
                    state.txtUsername = format!("");
                    state.txtPassword = format!("");
                }
            }
        }
        println!("State: {:#?}, ret: {:?}", &_maybe_state, &ret);
        ret
    }
}
