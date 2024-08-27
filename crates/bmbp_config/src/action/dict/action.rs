use crate::action::dict::bean::{BmbpCombo, BmbpCombos, BmbpDict, BmbpDisplay, BmbpDisplays, BatchReqVo, BatchComboVo};
use bmbp_http_type::{BmbpPageReq, BmbpResp};
use bmbp_http_type::PageData;
use bmbp_http_type::RespVo;
use salvo::{Depot, handler, Request, Response};
use crate::action::dict::service::BmbpDictService;


#[handler]
pub async fn find_dict_tree(
    req: &mut Request,
    resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<Vec<BmbpDict>>> {
    let params = req.parse_json::<BmbpDict>().await?;
    let data = BmbpDictService::find_dict_tree(depot, &params).await?;
    Ok(RespVo::ok_data_msg(data, "查询字典成功!".to_string()))
}

#[handler]
pub async fn find_dict_page(
    req: &mut Request,
    resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<PageData<BmbpDict>>> {
    let params = req.parse_json::<BmbpPageReq<BmbpDict>>().await?;
    let data = BmbpDictService::find_dict_page(depot, &params).await?;
    Ok(RespVo::ok_data_msg(data, "查询字典分页成功!".to_string()))
}

#[handler]
pub async fn find_dict_list(
    req: &mut Request,
    resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<Vec<BmbpDict>>> {
    let params = req.parse_json::<BmbpDict>().await?;
    let data = BmbpDictService::find_dict_list(depot, &params).await?;
    Ok(RespVo::ok_data_msg(data, "查询字典成功!".to_string()))
}

#[handler]
pub async fn find_dict_tree_ignore(
    req: &mut Request,
    resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<Vec<BmbpDict>>> {
    let params = req.parse_json::<BmbpDict>().await?;
    let data = BmbpDictService::find_dict_tree_ignore(depot, &params).await?;
    Ok(RespVo::ok_data_msg(data, "查询字典成功!".to_string()))
}

#[handler]
pub async fn find_dict_info(
    req: &mut Request,
    resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<BmbpDict>> {
    let dict_id = req.query::<String>("dataId");
    let data = BmbpDictService::find_dict_info(depot, dict_id.as_ref()).await?;
    Ok(RespVo::ok_data_msg(data, "查询字典成功!".to_string()))
}

#[handler]
pub async fn save_dict(
    req: &mut Request,
    resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<BmbpDict>> {
    let mut params = req.parse_json::<BmbpDict>().await?;
    let data = BmbpDictService::save_dict(depot, &mut params).await?;
    Ok(RespVo::ok_data_msg(data, "保存字典成功!".to_string()))
}

#[handler]
pub async fn insert_dict(
    req: &mut Request,
    resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<BmbpDict>> {
    let mut params = req.parse_json::<BmbpDict>().await?;
    let data = BmbpDictService::insert_dict(depot, &mut params).await?;
    Ok(RespVo::ok_data_msg(data, "新增字典成功!".to_string()))
}

#[handler]
pub async fn update_dict(
    req: &mut Request,
    resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<BmbpDict>> {
    let mut params = req.parse_json::<BmbpDict>().await?;
    let data = BmbpDictService::update_dict(depot, &mut params).await?;
    Ok(RespVo::ok_data_msg(data, "更新字典成功!".to_string()))
}

#[handler]
pub async fn enable_dict(
    req: &mut Request,
    resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<u64>> {
    let dict_id = req.query::<String>("dataId");
    let data = BmbpDictService::enable_dict(depot, dict_id.as_ref()).await?;
    Ok(RespVo::ok_data_msg(data, "启用字典成功!".to_string()))
}

#[handler]
pub async fn disable_dict(
    req: &mut Request,
    resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<u64>> {
    let dict_id = req.query::<String>("dataId");
    let data = BmbpDictService::disable_dict(depot, dict_id.as_ref()).await?;
    Ok(RespVo::ok_data_msg(data, "停用字典成功!".to_string()))
}

#[handler]
pub async fn batch_enable_dict(
    req: &mut Request,
    resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<u64>> {
    let dict_req = req.parse_json::<BatchReqVo>().await?;
    let data = BmbpDictService::batch_enable_dict(depot, &dict_req).await?;
    Ok(RespVo::ok_data_msg(data, "启用字典成功!".to_string()))
}


#[handler]
pub async fn batch_disable_dict(
    req: &mut Request,
    resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<u64>> {
    let dict_req = req.parse_json::<BatchReqVo>().await?;
    let data = BmbpDictService::batch_disable_dict(depot, &dict_req).await?;
    Ok(RespVo::ok_data_msg(data, "停用字典成功!".to_string()))
}


#[handler]
pub async fn remove_dict(
    req: &mut Request,
    resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<u64>> {
    let dict_id = req.query::<String>("dataId");
    let data = BmbpDictService::remove_dict(depot, dict_id.as_ref()).await?;
    Ok(RespVo::ok_data_msg(data, "删除字典成功!".to_string()))
}

#[handler]
pub async fn batch_remove_dict(
    req: &mut Request,
    resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<u64>> {
    let dict_req = req.parse_json::<BatchReqVo>().await?;
    let data = BmbpDictService::batch_remove_dict(depot, &dict_req).await?;
    Ok(RespVo::ok_data_msg(data, "删除字典成功!".to_string()))
}

#[handler]
pub async fn update_order(
    req: &mut Request,
    resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<u64>> {
    let params = req.parse_json::<BmbpDict>().await?;
    let data = BmbpDictService::update_order(depot, &params).await?;
    Ok(RespVo::ok_data_msg(data, "更新显示顺序成功!".to_string()))
}

#[handler]
pub async fn update_parent(
    req: &mut Request,
    resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<u64>> {
    let mut params = req.parse_json::<BmbpDict>().await?;
    let data = BmbpDictService::update_parent(depot, &mut params).await?;
    Ok(RespVo::ok_data_msg(data, "更新上级字典成功!".to_string()))
}

#[handler]
pub async fn find_dict_combo(
    req: &mut Request,
    resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<Vec<BmbpCombo>>> {
    let code = req.query::<String>("code");
    let cascade = req.query::<String>("cascade");
    let data = BmbpDictService::find_dict_combo(depot, code.as_ref(), cascade.as_ref()).await?;
    Ok(RespVo::ok_data_msg(data, "查询字典列表成功!".to_string()))
}

#[handler]
pub async fn find_dict_combos(
    req: &mut Request,
    resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<BmbpCombos>> {
    let combo_vo = req.parse_json::<BatchComboVo>().await?;
    let data = BmbpDictService::find_dict_combos(depot, &combo_vo).await?;
    Ok(RespVo::ok_data_msg(data, "查询字典列表成功!".to_string()))
}

#[handler]
pub async fn find_dict_display(
    req: &mut Request,
    resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<BmbpDisplay>> {
    let code = req.query::<String>("code");
    let cascade = req.query::<String>("cascade");
    let data = BmbpDictService::find_dict_display(depot, code.as_ref(), cascade.as_ref()).await?;
    Ok(RespVo::ok_data_msg(data, "查询字典转码成功!".to_string()))
}

#[handler]
pub async fn find_dict_displays(
    req: &mut Request,
    resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<BmbpDisplay>> {
    let code = req.parse_json::<BatchComboVo>().await?;
    let data = BmbpDictService::find_dict_displays(depot, &code).await?;
    Ok(RespVo::ok_data_msg(data, "查询字典转码成功!".to_string()))
}