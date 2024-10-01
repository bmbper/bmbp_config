use crate::action::vars::bean::BmbpVars;
use bmbp_http_type::PageData;
use bmbp_http_type::RespVo;
use bmbp_http_type::{BmbpPageReq, BmbpResp};
use salvo::{handler, Depot, Request, Response};

use super::bean::BatchReqVo;
use super::service::BmbpVarsService;

#[handler]
pub async fn vars_find_tree(
    req: &mut Request,
    _resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<Vec<BmbpVars>>> {
    let params = req.parse_json::<BmbpVars>().await?;
    let data = BmbpVarsService::vars_find_tree(depot, &params).await?;
    Ok(RespVo::ok_data_msg(data, "查询参数成功!".to_string()))
}

#[handler]
pub async fn vars_find_page(
    req: &mut Request,
    _resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<PageData<BmbpVars>>> {
    let params = req.parse_json::<BmbpPageReq<BmbpVars>>().await?;
    tracing::debug!("page params:{:#?}", params);
    let data = BmbpVarsService::vars_find_page(depot, &params).await?;
    Ok(RespVo::ok_data_msg(data, "查询参数分页成功!".to_string()))
}

#[handler]
pub async fn vars_find_list(
    req: &mut Request,
    _resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<Vec<BmbpVars>>> {
    let params = req.parse_json::<BmbpVars>().await?;
    let data = BmbpVarsService::vars_find_list(depot, &params).await?;
    Ok(RespVo::ok_data_msg(data, "查询参数成功!".to_string()))
}

#[handler]
pub async fn vars_find_tree_ignore(
    req: &mut Request,
    _resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<Vec<BmbpVars>>> {
    let params = req.parse_json::<BmbpVars>().await?;
    let data = BmbpVarsService::vars_find_tree_ignore(depot, &params).await?;
    Ok(RespVo::ok_data_msg(data, "查询参数成功!".to_string()))
}

#[handler]
pub async fn vars_find_info(
    req: &mut Request,
    _resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<BmbpVars>> {
    let vars_id = req.query::<String>("dataId");
    let data = BmbpVarsService::vars_find_info(depot, vars_id.as_ref()).await?;
    Ok(RespVo::ok_data_msg(data, "查询参数成功!".to_string()))
}

#[handler]
pub async fn vars_save(
    req: &mut Request,
    _resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<BmbpVars>> {
    let mut params = req.parse_json::<BmbpVars>().await?;
    let data = BmbpVarsService::vars_save(depot, &mut params).await?;
    Ok(RespVo::ok_data_msg(data, "保存参数成功!".to_string()))
}

#[handler]
pub async fn vars_insert(
    req: &mut Request,
    _resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<BmbpVars>> {
    let mut params = req.parse_json::<BmbpVars>().await?;
    let data = BmbpVarsService::vars_insert(depot, &mut params).await?;
    Ok(RespVo::ok_data_msg(data, "新增参数成功!".to_string()))
}

#[handler]
pub async fn vars_update(
    req: &mut Request,
    _resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<BmbpVars>> {
    let mut params = req.parse_json::<BmbpVars>().await?;
    let data = BmbpVarsService::vars_update(depot, &mut params).await?;
    Ok(RespVo::ok_data_msg(data, "更新参数成功!".to_string()))
}

#[handler]
pub async fn vars_enable(
    req: &mut Request,
    _resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<u64>> {
    let vars_id = req.query::<String>("dataId");
    let data = BmbpVarsService::vars_enable(depot, vars_id.as_ref()).await?;
    Ok(RespVo::ok_data_msg(data, "启用参数成功!".to_string()))
}

#[handler]
pub async fn vars_disable(
    req: &mut Request,
    _resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<u64>> {
    let vars_id = req.query::<String>("dataId");
    let data = BmbpVarsService::vars_disable(depot, vars_id.as_ref()).await?;
    Ok(RespVo::ok_data_msg(data, "停用参数成功!".to_string()))
}

#[handler]
pub async fn vars_batch_enable(
    req: &mut Request,
    _resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<u64>> {
    let vars_req = req.parse_json::<BatchReqVo>().await?;
    let data = BmbpVarsService::vars_batch_enable(depot, &vars_req).await?;
    Ok(RespVo::ok_data_msg(data, "启用参数成功!".to_string()))
}

#[handler]
pub async fn vars_batch_disable(
    req: &mut Request,
    _resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<u64>> {
    let vars_req = req.parse_json::<BatchReqVo>().await?;
    let data = BmbpVarsService::vars_batch_disable(depot, &vars_req).await?;
    Ok(RespVo::ok_data_msg(data, "停用参数成功!".to_string()))
}

#[handler]
pub async fn vars_remove(
    req: &mut Request,
    _resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<u64>> {
    let vars_id = req.query::<String>("dataId");
    let data = BmbpVarsService::vars_remove(depot, vars_id.as_ref()).await?;
    Ok(RespVo::ok_data_msg(data, "删除参数成功!".to_string()))
}

#[handler]
pub async fn vars_batch_remove(
    req: &mut Request,
    _resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<u64>> {
    let vars_req = req.parse_json::<BatchReqVo>().await?;
    let data = BmbpVarsService::vars_batch_remove(depot, &vars_req).await?;
    Ok(RespVo::ok_data_msg(data, "删除参数成功!".to_string()))
}

#[handler]
pub async fn vars_update_parent(
    req: &mut Request,
    _resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<u64>> {
    let mut params = req.parse_json::<BmbpVars>().await?;
    let data = BmbpVarsService::vars_update_parent(depot, &mut params).await?;
    Ok(RespVo::ok_data_msg(data, "更新上级参数成功!".to_string()))
}
