/*
extern crate chrono;
extern crate iron;
extern crate iron_sessionstorage;
extern crate rspten;
extern crate diesel;
extern crate urlencoded;
extern crate mustache;
#[macro_use]
extern crate rspten_derive;


use chrono::NaiveDateTime;
use diesel::prelude::*;
use iron::prelude::*;
use iron::status;

// use iron_sessionstorage;
use iron_sessionstorage::traits::*;
use rspten::*;
use urlencoded::UrlEncodedBody;

use std::collections::HashMap;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct LoginSessionState {
  pub username: String,
}

*/

use std::collections::HashMap;

extern crate mustache;
extern crate router;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

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
}

use mustache::MapBuilder;
use mustache::Template;
use rspten::RspAction;
use rspten::RspEvent;

struct HasDrop {
    data: i32,
}

impl Drop for HasDrop {
    fn drop(&mut self) {
        println!("Dropping!");
    }
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
        }
    }
    fn fill_data(
        data: MapBuilder,
        ev: &RspEvent,
        curr_key: &KeyI32,
        state: &Self,
        initial_state: &Self,
        curr_initial_state: &Self,
    ) -> MapBuilder {
        let mut data = data;
        let mut modified = false;
        let add_data = || data;

        html_button!(add_data, btnTest, "Test");
        html_text!(
            add_data,
            txt_text_message,
            state,
            curr_initial_state,
            modified
        );

        let a_disabled = if state.dd_testing % 2 == 0 {
            true
        } else {
            false
        };
        btnTest.borrow_mut().disabled = a_disabled;

        add_data()
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
