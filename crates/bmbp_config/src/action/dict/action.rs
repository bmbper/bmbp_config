use std::collections::HashMap;
use crate::action::dict::bean::{BmbpCombo, BmbpCombos, BmbpDict, BmbpDisplay, BmbpDisplays};
use bmbp_http_type::BmbpResp;
use bmbp_http_type::PageData;
use bmbp_http_type::RespVo;
use salvo::{Depot, handler, Request, Response};
use bmbp_app_util::parse_user_orm;
use crate::action::dict::service::BmbpDictService;


#[handler]
pub async fn find_dict_tree(
    req: &mut Request,
    resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<Vec<BmbpDict>>> {
    let params = req.parse_json::<BmbpDict>().await?;
    let data = BmbpDictService::find_dict_tree(depot, &params).await?;
    Ok(RespVo::ok_data_msg(
        data, "查询字典成功!".to_string(),
    ))
}

#[handler]
pub async fn find_dict_page(
    req: &mut Request,
    resp: &mut Response,
    _depot: &mut Depot,
) -> BmbpResp<RespVo<PageData<BmbpDict>>> {
    Ok(RespVo::ok_data_msg(
        Some(PageData::<BmbpDict>::default()),
        format!("thread id:{:#?}", ""),
    ))
}

#[handler]
pub async fn find_dict_list(
    req: &mut Request,
    resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<Vec<BmbpDict>>> {
    let (user_op, orm_op) = parse_user_orm(depot);
    Ok(RespVo::ok_data_msg(
        Some(vec![BmbpDict::default()]),
        format!("thread id:{:#?}", ""),
    ))
}

#[handler]
pub async fn find_dict_tree_ignore(
    req: &mut Request,
    resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<Vec<BmbpDict>>> {
    let (user_op, orm_op) = parse_user_orm(depot);
    Ok(RespVo::ok_data_msg(
        Some(vec![BmbpDict::default()]),
        format!("thread id:{:#?}", ""),
    ))
}

#[handler]
pub async fn find_dict_info(
    req: &mut Request,
    resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<Option<BmbpDict>>> {
    let (user_op, orm_op) = parse_user_orm(depot);
    Ok(RespVo::ok_data_msg(
        Some(None),
        format!("thread id:{:#?}", ""),
    ))
}

#[handler]
pub async fn save_dict(
    req: &mut Request,
    resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<Option<BmbpDict>>> {
    let (user_op, orm_op) = parse_user_orm(depot);
    Ok(RespVo::ok_data_msg(
        Some(None),
        format!("thread id:{:#?}", ""),
    ))
}

#[handler]
pub async fn insert_dict(
    req: &mut Request,
    resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<Option<BmbpDict>>> {
    let (user_op, orm_op) = parse_user_orm(depot);
    Ok(RespVo::ok_data_msg(
        Some(None),
        format!("thread id:{:#?}", ""),
    ))
}

#[handler]
pub async fn update_dict(
    req: &mut Request,
    resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<Option<BmbpDict>>> {
    let (user_op, orm_op) = parse_user_orm(depot);
    Ok(RespVo::ok_data_msg(
        Some(None),
        format!("thread id:{:#?}", ""),
    ))
}

#[handler]
pub async fn enable_dict(
    req: &mut Request,
    resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<u32>> {
    let (user_op, orm_op) = parse_user_orm(depot);
    Ok(RespVo::ok_data_msg(
        Some(0u32),
        format!("thread id:{:#?}", ""),
    ))
}

#[handler]
pub async fn batch_enable_dict(
    req: &mut Request,
    resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<u32>> {
    let (user_op, orm_op) = parse_user_orm(depot);
    Ok(RespVo::ok_data_msg(
        Some(0u32),
        format!("thread id:{:#?}", ""),
    ))
}

#[handler]
pub async fn disable_dict(
    req: &mut Request,
    resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<u32>> {
    let (user_op, orm_op) = parse_user_orm(depot);
    Ok(RespVo::ok_data_msg(
        Some(0u32),
        format!("thread id:{:#?}", ""),
    ))
}

#[handler]
pub async fn batch_disable_dict(
    req: &mut Request,
    resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<u32>> {
    let (user_op, orm_op) = parse_user_orm(depot);
    Ok(RespVo::ok_data_msg(
        Some(0u32),
        format!("thread id:{:#?}", ""),
    ))
}


#[handler]
pub async fn remove_dict(
    req: &mut Request,
    resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<u32>> {
    let (user_op, orm_op) = parse_user_orm(depot);
    Ok(RespVo::ok_data_msg(
        Some(0u32),
        format!("thread id:{:#?}", ""),
    ))
}

#[handler]
pub async fn batch_remove_dict(
    req: &mut Request,
    resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<u32>> {
    let (user_op, orm_op) = parse_user_orm(depot);
    Ok(RespVo::ok_data_msg(
        Some(0u32),
        format!("thread id:{:#?}", ""),
    ))
}

#[handler]
pub async fn update_order(
    req: &mut Request,
    resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<u32>> {
    let (user_op, orm_op) = parse_user_orm(depot);
    Ok(RespVo::ok_data_msg(
        Some(0u32),
        format!("thread id:{:#?}", ""),
    ))
}

#[handler]
pub async fn update_parent(
    req: &mut Request,
    resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<u32>> {
    let (user_op, orm_op) = parse_user_orm(depot);
    Ok(RespVo::ok_data_msg(
        Some(0u32),
        format!("thread id:{:#?}", ""),
    ))
}

#[handler]
pub async fn find_dict_combo(
    req: &mut Request,
    resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<Vec<BmbpCombo>>> {
    let (user_op, orm_op) = parse_user_orm(depot);
    Ok(RespVo::ok_data_msg(
        Some(vec![]),
        format!("thread id:{:#?}", ""),
    ))
}

#[handler]
pub async fn find_dict_combos(
    req: &mut Request,
    resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<BmbpCombos>> {
    let (user_op, orm_op) = parse_user_orm(depot);
    Ok(RespVo::ok_data_msg(
        Some(HashMap::new()),
        format!("thread id:{:#?}", ""),
    ))
}

#[handler]
pub async fn find_dict_display(
    req: &mut Request,
    resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<BmbpDisplay>> {
    let (user_op, orm_op) = parse_user_orm(depot);
    Ok(RespVo::ok_data_msg(
        Some(HashMap::new()),
        format!("thread id:{:#?}", ""),
    ))
}

#[handler]
pub async fn find_dict_displays(
    req: &mut Request,
    resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<BmbpDisplays>> {
    let (user_op, orm_op) = parse_user_orm(depot);
    Ok(RespVo::ok_data_msg(
        Some(HashMap::new()),
        format!("thread id:{:#?}", ""),
    ))
}