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

    let view_router = Router::with_path("bmbp/config/view")
        .push(Router::with_path("dict.view").get(dict_view))
        .push(Router::with_path("vars.view").get(vars_view))
        .push(Router::with_path("security.view").get(security_view))
        .push(Router::with_path("init.view").get(init_view));

    router = router.push(view_router);
    return router;
}
