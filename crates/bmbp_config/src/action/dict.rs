use bmbp_http_type::{RespVo};
use salvo::{handler, Request, Response};
use salvo::writing::Json;
use crate::action::bean::BmbpDict;

#[handler]
pub async fn find_dict_tree(
    req: &mut Request,
    resp: &mut Response,
) {
    resp.render(Json(RespVo::<BmbpDict>::ok_data_msg(Some(BmbpDict::default()), "dd".to_string())))
}
