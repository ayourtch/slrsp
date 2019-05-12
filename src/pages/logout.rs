use super::imports::*;

#[derive(RspTraits, Debug, Clone, Serialize, Deserialize)]
pub struct PageState {
}

type MyPageAuth = NoPageAuth;

impl RspState<(), MyPageAuth> for PageState {
    fn get_key(
        auth: &MyPageAuth,
        args: &HashMap<String, Vec<String>>,
        maybe_state: &Option<PageState>,
    ) -> () {
        ()
    }
    fn get_state(auth: &MyPageAuth, key: ()) -> PageState {
        PageState {
        }
    }
    fn fill_data(
        auth: &MyPageAuth,
        data: MapBuilder,
        ev: &RspEvent,
        curr_key: &(),
        state: &mut Self,
        initial_state: &Self,
        curr_initial_state: &Self,
    ) -> MapBuilder {
        let gd = || data;
        gd()
    }

    fn event_handler(
        req: &mut Request,
        auth: &MyPageAuth,
        _ev: &RspEvent,
        curr_key: &(),
        _maybe_state: &mut Option<PageState>,
        _maybe_initial_state: &Option<PageState>,
        _curr_initial_state: &PageState,
    ) -> RspAction<()> {
        req.session().clear().unwrap();
        rspten::RspAction::RedirectTo(format!("/"))
    }
}
