use std::thread;

use crate::action::dict::bean::BmbpDict;
use bmbp_http_type::BmbpResp;
use bmbp_http_type::PageData;
use bmbp_http_type::RespVo;
use salvo::{handler, Depot, Request, Response};

#[handler]
pub async fn find_dict_tree(
    req: &mut Request,
    resp: &mut Response,
    _depot: &mut Depot,
) -> BmbpResp<RespVo<Vec<BmbpDict>>> {
    let id = thread::current().id();
    Ok(RespVo::ok_data_msg(
        Some(vec![BmbpDict::default()]),
        format!("thread id:{:#?}", id),
    ))
}

#[handler]
pub async fn find_dict_page(
    req: &mut Request,
    resp: &mut Response,
    _depot: &mut Depot,
) -> BmbpResp<RespVo<PageData<BmbpDict>>> {
    let id = thread::current().id();
    Ok(RespVo::ok_data_msg(
        Some(PageData::<BmbpDict>::default()),
        format!("thread id:{:#?}", id),
    ))
}
