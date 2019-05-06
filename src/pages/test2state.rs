use super::imports::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageState {
    dd_testing: i32,
    txt_text_message: String,
}

impl RspState<i32, MyPageType> for PageState {
    fn get_template_name() -> String {
        "test2state".to_string()
    }
    fn get_key(args: &HashMap<String, Vec<String>>, maybe_state: &Option<PageState>) -> i32 {
        if let Some(st) = maybe_state {
            st.dd_testing
        } else {
            args.get("id2")
                .map_or(-1, |x| x[0].parse::<i32>().unwrap_or(-1))
        }
    }
    fn get_state(key: i32) -> PageState {
        println!("default state for second PageState with key: {:?}", &key);
        PageState {
            dd_testing: -1,
            txt_text_message: "test".to_string(),
        }
    }
    fn event_handler(
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