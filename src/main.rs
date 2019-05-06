use std::collections::HashMap;

extern crate mustache;
extern crate router;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate serde_frommap;

#[macro_use]
extern crate rspten;
#[macro_use]
extern crate rspten_derive;

struct MyPageType {}

impl rspten::RspPageType for MyPageType {}

use rspten::RspState;
use rspten::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct KeyI32 {
    id: Option<i32>,
}

#[derive(RspHandlers, Debug, Clone, Serialize, Deserialize)]
#[table = "TableFoo"]
struct TestState {
    dd_testing: i32,
    txt_text_message: String,
#[serde(default)]
    cbTestCheck: bool,
    ddMyDropdown: i32,
}

fn bool_false() -> bool {
    false
}

use mustache::MapBuilder;
use mustache::Template;
use rspten::RspAction;
use rspten::RspEvent;

fn dbh_get_dropdown(switchtype: i32) -> HtmlSelect<i32> {
    let mut dd: HtmlSelect<i32> = Default::default();
    dd.item(" --- ".into(), -1);
    for i in 1..23 {
      dd.item(&format!("item {}", i), i);
    }
    dd
}

fn dbh_get_testing_dropdown(switchtype: i32) -> HtmlSelect<i32> {
    let mut dd: HtmlSelect<i32> = Default::default();
    dd.item(" --- ".into(), -1);
    for i in 1..23 {
      dd.item(&format!("testing item {}", i), i);
    }
    dd
}

impl RspState<KeyI32, MyPageType> for TestState {
    fn get_template_name() -> String {
        "teststate".to_string()
    }
    fn get_key(args: &HashMap<String, Vec<String>>, _maybe_state: &Option<TestState>) -> KeyI32 {
        KeyI32 {
            id: args.get("id").map_or(None, |x| x[0].parse::<i32>().ok()),
        }
    }
    fn get_state(key: KeyI32) -> TestState {
        println!("default state for TestState with key: {:?}", &key);
        TestState {
            dd_testing: -1,
            txt_text_message: "test".to_string(),
            ddMyDropdown: key.id.unwrap_or(-1),
            cbTestCheck: true,
        }
    }
    fn fill_data(
        data: MapBuilder,
        ev: &RspEvent,
        curr_key: &KeyI32,
        state: &mut Self,
        initial_state: &Self,
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

        gd()
    }

    fn event_handler(
        _ev: &RspEvent,
        _curr_key: &KeyI32,
        _maybe_state: &mut Option<TestState>,
        _maybe_initial_state: &Option<TestState>,
        _curr_initial_state: &TestState,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Test2State {
    dd_testing: i32,
    txt_text_message: String,
}

impl RspState<i32, MyPageType> for Test2State {
    fn get_template_name() -> String {
        "test2state".to_string()
    }
    fn get_key(args: &HashMap<String, Vec<String>>, maybe_state: &Option<Test2State>) -> i32 {
        if let Some(st) = maybe_state {
            st.dd_testing
        } else {
            args.get("id2")
                .map_or(-1, |x| x[0].parse::<i32>().unwrap_or(-1))
        }
    }
    fn get_state(key: i32) -> Test2State {
        println!("default state for second Test2State with key: {:?}", &key);
        Test2State {
            dd_testing: -1,
            txt_text_message: "test".to_string(),
        }
    }
    fn event_handler(
        _ev: &RspEvent,
        curr_key: &i32,
        _maybe_state: &mut Option<Test2State>,
        _maybe_initial_state: &Option<Test2State>,
        _curr_initial_state: &Test2State,
    ) -> RspAction<i32> {
        if *curr_key == 42 {
            rspten::RspAction::RedirectTo("/".to_string())
        } else {
            rspten::RspAction::Render
        }
    }
}

fn main() {
    use router::Router;

    let mut router = Router::new();
    router.get("/", TestState::handler, "/");
    router.post("/", TestState::handler, "/");
    router.get("/x", Test2State::handler, "/x");

    let mut s = rspten::RspServer::new();

    s.run(router, "test service", 4480);
}
