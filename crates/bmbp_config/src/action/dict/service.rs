use bmbp_http_type::{BmbpResp, BmbpRespErr};
use salvo::Depot;
use bmbp_app_util::{parse_user_orm, valid_orm, valid_user};
use crate::action::dict::bean::BmbpDict;

pub struct BmbpDictService;

impl BmbpDictService {
    pub(crate) async fn find_dict_tree(depot: &mut Depot, params: &BmbpDict) -> BmbpResp<Option<Vec<BmbpDict>>> {
        let (user_op, orm_op) = parse_user_orm(depot);
        valid_user(user_op)?;
        valid_orm(orm_op)?;
        let user = user_op.unwrap();
        println!("{:#?}",user.get_id());
        if orm_op.is_none() {
            return Err(BmbpRespErr::err(Some("DB".to_string()), Some("未找到有效的数据库连接".to_string())));
        }
        Ok(None)
    }
}