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
    let action_router = Router::new().push(
        Router::with_path("dict")
            .push(Router::with_path("tree").post(find_dict_tree))
            .push(Router::with_path("page").post(find_dict_page))
            .push(Router::with_path("list").post(find_dict_list))
            .push(Router::with_path("info").post(find_dict_info))
            .push(Router::with_path("tree/ignore").post(find_dict_tree_ignore))
            .push(Router::with_path("save").post(save_dict))
            .push(Router::with_path("insert").post(insert_dict))
            .push(Router::with_path("update").post(update_dict))
            .push(Router::with_path("enable").post(enable_dict))
            .push(Router::with_path("batch/enable").post(batch_enable_dict))
            .push(Router::with_path("disable").post(disable_dict))
            .push(Router::with_path("batch/disable").post(batch_disable_dict))
            .push(Router::with_path("remove").post(remove_dict))
            .push(Router::with_path("batch/remove").post(batch_remove_dict))
            .push(Router::with_path("update/parent").post(update_parent))
            .push(Router::with_path("update_order").post(update_order))
            .push(Router::with_path("combo").post(find_dict_combo))
            .push(Router::with_path("combos").post(find_dict_combos))
            .push(Router::with_path("display").post(find_dict_display))
            .push(Router::with_path("displays").post(find_dict_displays))
            .push(Router::with_path("index.view").get(dict_view))
    );
    router = router.push(action_router);

    // view router
    let view_router = Router::with_path("bmbp/config/view")
        .push(Router::with_path("vars.view").get(vars_view))
        .push(Router::with_path("security.view").get(security_view))
        .push(Router::with_path("init.view").get(init_view));

    router = router.push(view_router);
    return router;
}
