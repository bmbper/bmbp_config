use crate::action::dict::*;
use crate::init::build_bmbp_static_router;
use crate::view::*;
use salvo::prelude::*;

mod action;
mod ctx;
mod init;
mod view;

pub fn build_bmbp_config_router() -> Router {
    let mut router = Router::new();
    router = router.push(build_bmbp_static_router());

    // action router
    let action_router = Router::with_path("/bmbp/config/action").push(
        Router::with_path("dict")
            .push(Router::with_path("find_tree.action").post(find_dict_tree))
            .push(Router::with_path("find_page.action").post(find_dict_page)),
    );
    router = router.push(action_router);

    // view router
    let view_router = Router::with_path("bmbp/config/view")
        .push(Router::with_path("dict.view").get(dict_view))
        .push(Router::with_path("vars.view").get(vars_view))
        .push(Router::with_path("security.view").get(security_view))
        .push(Router::with_path("init.view").get(init_view));

    router = router.push(view_router);
    return router;
}
