use crate::action::bean::BmbpDict;
use bmbp_http_type::BmbpResp;
use bmbp_http_type::RespVo;
use salvo::{handler, Request, Response};

#[handler]
pub async fn find_dict_tree(
    req: &mut Request,
    resp: &mut Response,
) -> BmbpResp<RespVo<Vec<BmbpDict>>> {
    Ok(RespVo::ok_data_msg(
        Some(vec![BmbpDict::default()]),
        "功能占位".to_string(),
    ))
}
