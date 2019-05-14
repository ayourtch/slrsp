use super::imports::*;

#[derive(RspTraits, Debug, Clone, Serialize, Deserialize)]
pub struct PageState {
    dd_testing: i32,
    txt_text_message: String,
}

type MyPageAuth = NoPageAuth;

impl RspState<i32, MyPageAuth> for PageState {
    fn get_key(
        _auth: &MyPageAuth,
        args: &HashMap<String, Vec<String>>,
        maybe_state: &Option<PageState>,
    ) -> i32 {
        if let Some(st) = maybe_state {
            st.dd_testing
        } else {
            args.get("id2")
                .map_or(-1, |x| x[0].parse::<i32>().unwrap_or(-1))
        }
    }
    fn get_state(_auth: &MyPageAuth, key: i32) -> PageState {
        println!("default state for second PageState with key: {:?}", &key);
        PageState {
            dd_testing: -1,
            txt_text_message: "test".to_string(),
        }
    }
    fn event_handler(
        _req: &mut Request,
        _auth: &MyPageAuth,
        _ev: &RspEvent,
        curr_key: &i32,
        _maybe_state: &mut Option<PageState>,
        _maybe_initial_state: &Option<PageState>,
        _curr_initial_state: &PageState,
    ) -> RspAction<i32> {
        if *curr_key == 42 {
            rspten::RspAction::RedirectTo("/".to_string())
        } else {
            rspten::RspAction::Render
        }
    }
}
