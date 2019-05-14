#![allow(non_snake_case)]

use super::imports::*;
use html_helpers::dbh_get_dropdown;
use html_helpers::dbh_get_testing_dropdown;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyI32 {
    id: Option<i32>,
}

#[derive(RspHandlers, RspTraits, Debug, Clone, Serialize, Deserialize)]
#[table = "TableFoo"]
pub struct PageState {
    dd_testing: i32,
    txt_text_message: String,
    #[serde(default)]
    cbTestCheck: bool,
    ddMyDropdown: i32,
}

type MyPageAuth = NoPageAuth;

impl RspState<KeyI32, MyPageAuth> for PageState {
    fn get_key(
        _auth: &MyPageAuth,
        args: &HashMap<String, Vec<String>>,
        _maybe_state: &Option<PageState>,
    ) -> KeyI32 {
        KeyI32 {
            id: args.get("id").map_or(None, |x| x[0].parse::<i32>().ok()),
        }
    }
    fn get_state(_auth: &MyPageAuth, key: KeyI32) -> PageState {
        println!("default state for PageState with key: {:?}", &key);
        PageState {
            dd_testing: -1,
            txt_text_message: "test".to_string(),
            ddMyDropdown: key.id.unwrap_or(-1),
            cbTestCheck: true,
        }
    }
    fn fill_data(
        _auth: &MyPageAuth,
        data: MapBuilder,
        _ev: &RspEvent,
        curr_key: &KeyI32,
        state: &mut Self,
        _initial_state: &Self,
        curr_initial_state: &Self,
    ) -> MapBuilder {
        let mut modified = false;
        let gd = || data;

        html_button!(gd, btnTest, "Test");
        html_text!(gd, txt_text_message, state, curr_initial_state, modified);
        html_check!(gd, cbTestCheck, state, curr_initial_state, modified);

        html_select!(
            gd,
            ddMyDropdown,
            dbh_get_dropdown(curr_key.id.unwrap_or(-1)),
            state,
            curr_initial_state,
            modified
        );
        html_select!(
            gd,
            dd_testing,
            dbh_get_testing_dropdown(curr_key.id.unwrap_or(-1)),
            state,
            curr_initial_state,
            modified
        );

        btnTest.borrow_mut().disabled = if state.dd_testing % 2 == 0 {
            true
        } else {
            false
        };

        let gd = || gd().insert("modified", &modified).unwrap();

        gd()
    }

    fn event_handler(
        _req: &mut Request,
        _auth: &MyPageAuth,
        _ev: &RspEvent,
        _curr_key: &KeyI32,
        _maybe_state: &mut Option<PageState>,
        _maybe_initial_state: &Option<PageState>,
        _curr_initial_state: &PageState,
    ) -> RspAction<KeyI32> {
        if _ev.event == "submit" {
            if let Some(state) = _maybe_state {
                let tgt = &_ev.target[..];
                match tgt {
                    "_eq" => {
                        state.txt_text_message =
                            format!("Pressed eq when state is {}", state.dd_testing);
                    }
                    "_lt" => {
                        state.dd_testing = state.dd_testing - 1;
                    }
                    "_gt" => {
                        state.dd_testing = state.dd_testing + 1;
                    }
                    _ => {}
                }
            }
        }
        rspten::RspAction::Render
    }
}
