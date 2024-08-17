use std::thread;

use bmbp_http_type::BmbpResp;
use bmbp_http_type::RespVo;
use salvo::{Depot, handler, Request, Response};
use crate::action::dict::bean::BmbpDict;

#[handler]
pub async fn find_dict_tree(
    req: &mut Request,
    resp: &mut Response,
    _depot:&mut Depot
) -> BmbpResp<RespVo<Vec<BmbpDict>>> {
    let id = thread::current().id();
    Ok(RespVo::ok_data_msg(
        Some(vec![BmbpDict::default()]),
        format!("thread id:{:#?}",id),
    ))
}
