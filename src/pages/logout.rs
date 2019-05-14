use super::imports::*;

#[derive(RspTraits, Debug, Clone, Serialize, Deserialize)]
pub struct PageState {}

type MyPageAuth = NoPageAuth;

impl RspState<(), MyPageAuth> for PageState {
    fn get_key(
        _auth: &MyPageAuth,
        _args: &HashMap<String, Vec<String>>,
        _maybe_state: &Option<PageState>,
    ) -> () {
        ()
    }
    fn get_state(_auth: &MyPageAuth, _key: ()) -> PageState {
        PageState {}
    }
    fn fill_data(
        _auth: &MyPageAuth,
        data: MapBuilder,
        _ev: &RspEvent,
        _curr_key: &(),
        _state: &mut Self,
        _initial_state: &Self,
        _curr_initial_state: &Self,
    ) -> MapBuilder {
        let gd = || data;
        gd()
    }

    fn event_handler(
        req: &mut Request,
        _auth: &MyPageAuth,
        _ev: &RspEvent,
        _curr_key: &(),
        _maybe_state: &mut Option<PageState>,
        _maybe_initial_state: &Option<PageState>,
        _curr_initial_state: &PageState,
    ) -> RspAction<()> {
        req.session().clear().unwrap();
        rspten::RspAction::RedirectTo(format!("/"))
    }
}
