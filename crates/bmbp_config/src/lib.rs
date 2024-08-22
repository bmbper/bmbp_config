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
            .push(Router::with_path("find_page.action").post(find_dict_page))
            .push(Router::with_path("find_list.action").post(find_dict_list))
            .push(Router::with_path("find_info.action").post(find_dict_info))
            .push(Router::with_path("find_ignore_tree.action").post(find_dict_tree_ignore))
            .push(Router::with_path("save.action").post(save_dict))
            .push(Router::with_path("insert.action").post(insert_dict))
            .push(Router::with_path("update.action").post(update_dict))
            .push(Router::with_path("enable.action").post(enable_dict))
            .push(Router::with_path("batch_enable.action").post(batch_enable_dict))
            .push(Router::with_path("disable.action").post(disable_dict))
            .push(Router::with_path("batch_disable.action").post(batch_disable_dict))
            .push(Router::with_path("remove.action").post(remove_dict))
            .push(Router::with_path("batch_remove.action").post(batch_remove_dict))
            .push(Router::with_path("update_parent.action").post(update_parent))
            .push(Router::with_path("update_order.action").post(update_order))
            .push(Router::with_path("find_combo.action").post(find_dict_combo))
            .push(Router::with_path("find_combos.action").post(find_dict_combos))
            .push(Router::with_path("find_display.action").post(find_dict_display))
            .push(Router::with_path("find_displays.action").post(find_dict_displays))
        ,
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
