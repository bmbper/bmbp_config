use bmbp_http_type::{BmbpPageReq, BmbpResp, BmbpRespErr, PageData};
use bmbp_rdbc_orm::{DeleteWrapper, InsertWrapper, QueryWrapper, RdbcResult, RdbcTableFilter, RdbcTableWrapper, UpdateWrapper};
use bmbp_rdbc_type::RdbcTable;
use bmbp_util::{BmbpId, BmbpTreeUtil};
use salvo::Depot;
use bmbp_rdbc_type::RdbcIdent;
use bmbp_app_util::parse_orm;

use crate::action::dict::bean::{BatchComboVo, BatchReqVo, BmbpCombo, BmbpCombos, BmbpDict, BmbpDictColumn, BmbpDisplay, BmbpDisplays};

pub struct BmbpDictService;


impl BmbpDictService {
    pub(crate) async fn find_dict_tree(depot: &mut Depot, params: &BmbpDict) -> BmbpResp<Option<Vec<BmbpDict>>> {
        let dict_vec = Self::find_dict_list(depot, params).await?;
        if let Some(dic) = dict_vec {
            Ok(Some(BmbpTreeUtil::build_tree::<BmbpDict>(dic)))
        } else {
            Ok(None)
        }
    }
    pub(crate) async fn find_dict_page(depot: &mut Depot, page_req: &BmbpPageReq<BmbpDict>) -> BmbpResp<Option<PageData<BmbpDict>>> {
        let query_wrapper = Self::build_dict_query_wrapper(depot, page_req.get_params()).await?;
        let orm = parse_orm(depot)?;
        return match orm.select_page_by_query::<BmbpDict>(page_req.get_page_no().clone(), page_req.get_page_size().clone(), &query_wrapper).await {
            Ok(mut orm_page) => {
                let orm_page_data = orm_page.data_take();
                let resp_page = PageData::new(orm_page.page_num().clone() as u32, orm_page.page_size().clone() as u32, orm_page.total().clone() as u32, orm_page_data.unwrap_or(vec![]));
                Ok(Some(resp_page))
            }
            Err(err) => {
                Err(BmbpRespErr::err(Some("DB".to_string()), Some(err.get_msg())))
            }
        };
    }
    pub(crate) async fn find_dict_list(depot: &mut Depot, params: &BmbpDict) -> BmbpResp<Option<Vec<BmbpDict>>> {
        let query_wrapper = Self::build_dict_query_wrapper(depot, Some(params)).await?;
        let orm = parse_orm(depot)?;
        match orm.select_list_by_query::<BmbpDict>(&query_wrapper).await {
            Ok(dict) => {
                Ok(dict)
            }
            Err(err) => {
                Err(BmbpRespErr::err(Some("DB".to_string()), Some(err.get_msg())))
            }
        }
    }
    pub(crate) async fn find_dict_tree_ignore(depot: &mut Depot, params: &BmbpDict) -> BmbpResp<Option<Vec<BmbpDict>>> {
        let mut query_wrapper = QueryWrapper::new_from::<BmbpDict>();
        if let Some(dict_id) = params.get_data_id() {
            let dict = Self::find_dict_info(depot, Some(dict_id)).await?;
            if dict.is_none() {
                return Err(BmbpRespErr::err(Some("DB".to_string()), Some("未找到字典信息".to_string())));
            }
            query_wrapper.not_like_left(BmbpDictColumn::DictCodePath, dict_id.clone());
        }
        if let Some(dict_code) = params.get_dict_code() {
            query_wrapper.not_like_left(BmbpDictColumn::DictCodePath, dict_code.clone());
        }
        if let Some(dict_parent_code) = params.get_dict_parent_code() {
            query_wrapper.not_like_left(BmbpDictColumn::DictCodePath, dict_parent_code.clone());
        }
        let orm = parse_orm(depot)?;
        match orm.select_list_by_query::<BmbpDict>(&query_wrapper).await {
            Ok(dic) => {
                if let Some(dic) = dic {
                    Ok(Some(BmbpTreeUtil::build_tree::<BmbpDict>(dic)))
                } else {
                    Ok(None)
                }
            }
            Err(err) => {
                Err(BmbpRespErr::err(Some("DB".to_string()), Some(err.get_msg())))
            }
        }
    }


    pub(crate) async fn find_dict_info(depot: &mut Depot, params: Option<&String>) -> BmbpResp<Option<BmbpDict>> {
        let mut query_wrapper = QueryWrapper::new_from::<BmbpDict>();
        if params.is_none() || params.as_ref().unwrap().is_empty() {
            return Err(BmbpRespErr::err(Some("VALID".to_string()), Some("请传入字典标识".to_string())));
        }
        query_wrapper.eq_(BmbpDictColumn::DataId, params.unwrap().clone());
        let orm = parse_orm(depot)?;
        match orm.select_one_by_query::<BmbpDict>(&query_wrapper).await {
            Ok(dict) => {
                Ok(dict)
            }
            Err(err) => {
                Err(BmbpRespErr::err(Some("DB".to_string()), Some(err.get_msg())))
            }
        }
    }
    pub(crate) async fn find_dict_info_by_alias(depot: &mut Depot, alias: Option<&String>) -> BmbpResp<Option<BmbpDict>> {
        if alias.is_none() || alias.as_ref().unwrap().is_empty() {
            return Err(BmbpRespErr::err(Some("VALID".to_string()), Some("请传入字典编码".to_string())));
        }
        let mut query_wrapper = QueryWrapper::new_from::<BmbpDict>();
        query_wrapper.eq_(BmbpDictColumn::DictAlias, alias.unwrap().clone());
        let orm = parse_orm(depot)?;
        match orm.select_one_by_query::<BmbpDict>(&query_wrapper).await {
            Ok(dict) => {
                Ok(dict)
            }
            Err(err) => {
                Err(BmbpRespErr::err(Some("DB".to_string()), Some(err.get_msg())))
            }
        }
    }
    pub(crate) async fn find_dict_info_in_alias(depot: &mut Depot, alias: &[String]) -> BmbpResp<Option<Vec<BmbpDict>>> {
        if alias.is_empty() {
            return Err(BmbpRespErr::err(Some("VALID".to_string()), Some("请传入字典编码".to_string())));
        }
        let mut query_wrapper = QueryWrapper::new_from::<BmbpDict>();
        query_wrapper.in_v_slice(BmbpDictColumn::DictAlias, alias);
        let orm = parse_orm(depot)?;
        match orm.select_list_by_query::<BmbpDict>(&query_wrapper).await {
            Ok(dic) => {
                Ok(dic)
            }
            Err(err) => {
                return Err(BmbpRespErr::err(Some("DB".to_string()), Some(err.get_msg())));
            }
        }
    }

    pub(crate) async fn save_dict(depot: &mut Depot, params: &mut BmbpDict) -> BmbpResp<Option<BmbpDict>> {
        let mut dict_info = Self::find_dict_info(depot, params.get_data_id().as_ref()).await?;
        if dict_info.is_none() {
            Self::insert_dict(depot, params).await
        } else {
            Self::update_dict(depot, params).await
        }
    }

    pub(crate) async fn insert_dict(depot: &mut Depot, params: &mut BmbpDict) -> BmbpResp<Option<BmbpDict>> {
        if params.get_data_id().is_none() {
            params.set_data_id(Some(BmbpId::simple_uuid()));
        }
        let insert_wrapper = InsertWrapper::new();
        let orm = parse_orm(depot)?;
        return match orm.execute_insert(&insert_wrapper).await {
            Ok(_) => {
                Self::find_dict_info(depot, params.get_data_id().as_ref()).await
            }
            Err(err) => {
                Err(BmbpRespErr::err(Some("DB".to_string()), Some(err.get_msg())))
            }
        };
    }


    pub(crate) async fn update_dict(depot: &mut Depot, params: &mut BmbpDict) -> BmbpResp<Option<BmbpDict>> {
        if params.get_data_id().is_none() {
            return Err(BmbpRespErr::err(Some("VALID".to_string()), Some("请传入字典标识".to_string())));
        }
        let update_wrapper = UpdateWrapper::new();
        let orm = parse_orm(depot)?;
        return match orm.execute_update(&update_wrapper).await {
            Ok(_) => {
                Self::find_dict_info(depot, params.get_data_id().as_ref()).await
            }
            Err(err) => {
                Err(BmbpRespErr::err(Some("DB".to_string()), Some(err.get_msg())))
            }
        };
    }

    pub(crate) async fn enable_dict(depot: &mut Depot, params: Option<&String>) -> BmbpResp<Option<u64>> {
        let dict_info = Self::find_dict_info(depot, params).await?;
        if dict_info.is_none() {
            return Err(BmbpRespErr::err(Some("REQUEST".to_string()), Some("未找到字典信息".to_string())));
        }
        let mut update_wrapper = UpdateWrapper::new();
        update_wrapper.set(BmbpDictColumn::DataStatus, "1");
        update_wrapper.table(BmbpDict::get_table().get_ident());
        update_wrapper.eq_(BmbpDictColumn::DataId, params.unwrap().clone());
        Self::execute_update(depot, &update_wrapper).await
    }

    pub(crate) async fn batch_enable_dict(depot: &mut Depot, params: &BatchReqVo) -> BmbpResp<Option<u64>> {
        let mut update_wrapper = UpdateWrapper::new();
        update_wrapper.set(BmbpDictColumn::DataStatus, "1");
        update_wrapper.table(BmbpDict::get_table().get_ident());
        update_wrapper.in_v(BmbpDictColumn::DataId, params.get_ids().clone().unwrap_or(vec![]));
        Self::execute_update(depot, &update_wrapper).await
    }

    pub(crate) async fn disable_dict(depot: &mut Depot, params: Option<&String>) -> BmbpResp<Option<u64>> {
        let dict_info = Self::find_dict_info(depot, params).await?;
        if dict_info.is_none() {
            return Err(BmbpRespErr::err(Some("REQUEST".to_string()), Some("未找到字典信息".to_string())));
        }
        let mut update_wrapper = UpdateWrapper::new();
        update_wrapper.set(BmbpDictColumn::DataStatus, "0");
        update_wrapper.table(BmbpDict::get_table().get_ident());
        update_wrapper.eq_(BmbpDictColumn::DataId, params.unwrap().clone());
        Self::execute_update(depot, &update_wrapper).await
    }

    pub(crate) async fn batch_disable_dict(depot: &mut Depot, params: &BatchReqVo) -> BmbpResp<Option<u64>> {
        let mut update_wrapper = UpdateWrapper::new();
        update_wrapper.set(BmbpDictColumn::DataStatus, "0");
        update_wrapper.table(BmbpDict::get_table().get_ident());
        update_wrapper.in_v(BmbpDictColumn::DataId, params.get_ids().clone().unwrap_or(vec![]));
        Self::execute_update(depot, &update_wrapper).await
    }

    pub(crate) async fn remove_dict(depot: &mut Depot, params: Option<&String>) -> BmbpResp<Option<u64>> {
        let dict_info = Self::find_dict_info(depot, params).await?;
        if dict_info.is_none() {
            return Err(BmbpRespErr::err(Some("REQUEST".to_string()), Some("未找到字典信息".to_string())));
        }
        let mut wrapper = DeleteWrapper::new();
        wrapper.table(BmbpDict::get_table().get_ident());
        wrapper.eq_(BmbpDictColumn::DataId, params.clone());
        Self::execute_delete(depot, &wrapper).await
    }

    pub(crate) async fn batch_remove_dict(depot: &mut Depot, params: &BatchReqVo) -> BmbpResp<Option<u64>> {
        let mut wrapper = DeleteWrapper::new();
        wrapper.table(BmbpDict::get_table().get_ident());
        wrapper.in_v(BmbpDictColumn::DataId, params.get_ids().clone().unwrap_or(vec![]));
        Self::execute_delete(depot, &wrapper).await
    }

    pub(crate) async fn update_order(depot: &mut Depot, params: &BmbpDict) -> BmbpResp<Option<u64>> {
        let dict_info = Self::find_dict_info(depot, params.get_data_id().as_ref()).await?;
        if dict_info.is_none() {
            return Err(BmbpRespErr::err(Some("REQUEST".to_string()), Some("未找到字典信息".to_string())));
        }
        let mut update_wrapper = UpdateWrapper::new();
        update_wrapper.set(BmbpDictColumn::DataSort, params.get_data_sort().clone().unwrap_or(0i32));
        update_wrapper.table(BmbpDict::get_table().get_ident());
        update_wrapper.eq_(BmbpDictColumn::DataId, params.get_data_id().as_ref().unwrap());
        Self::execute_update(depot, &update_wrapper).await
    }

    pub(crate) async fn update_parent(depot: &mut Depot, params: &BmbpDict) -> BmbpResp<Option<u64>> {
        let dict_info = Self::find_dict_info(depot, params.get_data_id().as_ref()).await?;
        if dict_info.is_none() {
            return Err(BmbpRespErr::err(Some("REQUEST".to_string()), Some("未找到字典信息".to_string())));
        }
        let mut update_wrapper = UpdateWrapper::new();
        update_wrapper.set(BmbpDictColumn::DataSort, params.get_data_sort().clone().unwrap_or(0i32));
        update_wrapper.table(BmbpDict::get_table().get_ident());
        update_wrapper.eq_(BmbpDictColumn::DataId, params.get_data_id().as_ref().unwrap());
        Self::execute_update(depot, &update_wrapper).await
    }

    pub(crate) async fn find_dict_combo(depot: &mut Depot, alias: Option<&String>, cascade: Option<&String>) -> BmbpResp<Option<Vec<BmbpCombo>>> {
        if alias.is_none() || alias.as_ref().unwrap().is_empty() {
            return Err(BmbpRespErr::err(Some("REQUEST".to_string()), Some("字典编码不能为空".to_string())));
        }
        let code = vec![alias.unwrap().clone()];
        let dict_vec = Self::find_dict_children_by_alias(depot, code.as_slice(), cascade).await?;
        Self::convert_dict_to_combo(dict_vec)
    }

    pub(crate) async fn find_dict_combos(depot: &mut Depot, params: &BatchComboVo) -> BmbpResp<Option<BmbpCombos>> {
        let code = params.get_codes().as_ref().unwrap_or(&vec![]).clone();
        let cascade = params.get_cascade().as_ref().clone();
        let dict_vec = Self::find_dict_children_by_alias(depot, code.as_slice(), cascade).await?;

        let dict_vec = dict_vec.unwrap_or(vec![]);
        let dict_vec = BmbpTreeUtil::build_tree::<BmbpDict>(dict_vec);
        Self::convert_dict_to_combos(code.as_slice(), dict_vec)
    }

    pub(crate) async fn find_dict_display(depot: &mut Depot, alias: Option<&String>, cascade: Option<&String>) -> BmbpResp<Option<BmbpDisplay>> {
        if alias.is_none() || alias.as_ref().unwrap().is_empty() {
            return Err(BmbpRespErr::err(Some("REQUEST".to_string()), Some("字典编码不能为空".to_string())));
        }
        let code = vec![alias.unwrap().clone()];
        let dict_vec = Self::find_dict_children_by_alias(depot, code.as_slice(), cascade).await?;
        Self::convert_dict_to_display(dict_vec)
    }

    pub(crate) async fn find_dict_displays(depot: &mut Depot, params: &BatchComboVo) -> BmbpResp<Option<BmbpDisplay>> {
        let code = params.get_codes().as_ref().unwrap_or(&vec![]).clone();
        let cascade = params.get_cascade().as_ref().clone();
        let mut dict_vec = Self::find_dict_children_by_alias(depot, code.as_slice(), cascade).await?;

        let dict_vec = Self::find_dict_children_by_alias(depot, code.as_slice(), cascade).await?;

        let mut dict_vec = dict_vec.unwrap_or(vec![]);
        dict_vec = BmbpTreeUtil::build_tree::<BmbpDict>(dict_vec);
        Self::convert_dict_to_displays(code.as_slice(), dict_vec)
    }

    async fn build_dict_query_wrapper(depot: &mut Depot, params_op: Option<&BmbpDict>) -> BmbpResp<QueryWrapper> {
        let mut query_wrapper = QueryWrapper::new_from::<BmbpDict>();
        if let Some(params) = params_op {
            if let Some(dict_id) = params.get_data_id() {
                let dict = Self::find_dict_info(depot, Some(dict_id)).await?;
                if dict.is_none() {
                    return Err(BmbpRespErr::err(Some("DB".to_string()), Some("未找到字典信息".to_string())));
                }
                query_wrapper.like_left(BmbpDictColumn::DictCodePath, dict_id.clone());
            }
            if let Some(dict_code) = params.get_dict_code() {
                query_wrapper.like_left(BmbpDictColumn::DictCodePath, dict_code.clone());
            }
            if let Some(dict_parent_code) = params.get_dict_parent_code() {
                query_wrapper.like_left(BmbpDictColumn::DictCodePath, dict_parent_code.clone());
            }
        }
        Ok(query_wrapper)
    }
    async fn execute_update(depot: &mut Depot, update_wrapper: &UpdateWrapper) -> BmbpResp<Option<u64>> {
        let orm = parse_orm(depot)?;
        return match orm.execute_update(&update_wrapper).await {
            Ok(row_count) => {
                Ok(Some(row_count))
            }
            Err(err) => {
                Err(BmbpRespErr::err(Some("DB".to_string()), Some(err.get_msg())))
            }
        };
    }
    async fn execute_delete(depot: &mut Depot, delete_wrapper: &DeleteWrapper) -> BmbpResp<Option<u64>> {
        let orm = parse_orm(depot)?;
        return match orm.execute_delete(&delete_wrapper).await {
            Ok(row_count) => {
                Ok(Some(row_count))
            }
            Err(err) => {
                Err(BmbpRespErr::err(Some("DB".to_string()), Some(err.get_msg())))
            }
        };
    }
    async fn find_dict_children_by_alias(depot: &mut Depot, alias: &[String], cascade: Option<&String>) -> BmbpResp<Option<Vec<BmbpDict>>> {
        let dict_vec_op = Self::find_dict_info_in_alias(depot, alias).await?;
        if dict_vec_op.is_none() || dict_vec_op.as_ref().unwrap().is_empty() {
            return Err(BmbpRespErr::err(Some("REQUEST".to_string()), Some("未找到字典信息".to_string())));
        }
        let mut dict_code_vec = vec![];
        for item in dict_vec_op.as_ref().unwrap() {
            dict_code_vec.push(item.get_dict_code().as_ref().unwrap().clone());
        }
        let mut query_wrapper = QueryWrapper::new_from::<BmbpDict>();
        if cascade.is_some() && (cascade.as_ref().unwrap().as_str().eq("1") || cascade.as_ref().unwrap().as_str().eq("true")) {
            query_wrapper.or();
            for code in dict_code_vec.as_slice() {
                query_wrapper.like(BmbpDictColumn::DictCodePath, code.clone());
            }
        } else {
            query_wrapper.in_v_slice(BmbpDictColumn::DictParentCode, dict_code_vec.as_slice());
        }
        let orm = parse_orm(depot)?;
        match orm.select_list_by_query::<BmbpDict>(&query_wrapper).await {
            Ok(dict_vec) => {
                Ok(dict_vec)
            }
            Err(err) => {
                Err(BmbpRespErr::err(Some("DB".to_string()), Some(err.get_msg())))
            }
        }
    }
    fn convert_dict_to_combo(dict_vec: Option<Vec<BmbpDict>>) -> BmbpResp<Option<Vec<BmbpCombo>>> {
        match dict_vec {
            Some(v) => {
                let mut combo_vec = vec![];
                for dict in v {
                    let mut combo = BmbpCombo::new();
                    combo.set_value(dict.get_dict_value().clone());
                    combo.set_label(dict.get_dict_name().clone());
                    combo_vec.push(combo);
                }
                Ok(Some(combo_vec))
            }
            None => {
                Ok(None)
            }
        }
    }
    fn convert_dict_to_display(dict_vec: Option<Vec<BmbpDict>>) -> BmbpResp<Option<BmbpDisplay>> {
        match dict_vec {
            Some(v) => {
                let mut display = BmbpDisplay::new();
                for dict in v {
                    display.insert(dict.get_dict_value().as_ref().unwrap().clone(), dict.get_dict_name().as_ref().unwrap().clone());
                }
                Ok(Some(display))
            }
            None => {
                Ok(None)
            }
        }
    }
    fn convert_dict_to_combos(codes: &[String], dict_vec: Vec<BmbpDict>) -> BmbpResp<Option<BmbpCombos>> {
        let mut combos = BmbpCombos::new();
        for dict in dict_vec.as_slice() {
            let code = dict.get_dict_alias().as_ref().unwrap();
            let combo_vec = convert_to_vec(dict.get_dict_children().as_ref().unwrap().as_slice());
            combos.insert(code.to_string(), combo_vec);
        }
        Ok(Some(combos))
    }
    fn convert_dict_to_displays(code: &[String], dict_vec: Vec<BmbpDict>) -> BmbpResp<Option<BmbpDisplay>> {
        let display = BmbpDisplay::new();
        // TODO
        Ok(Some(display))
    }
}

fn convert_to_vec(dict_vec: &[BmbpDict]) -> Vec<BmbpCombo> {
    let mut bmbp_combo = vec![];
    for item in dict_vec {
        let mut child_combo = vec![];
        if item.get_dict_children().is_some() && !item.get_dict_children().as_ref().unwrap().is_empty() {
            child_combo = convert_to_vec(item.get_dict_children().as_ref().unwrap().as_slice());
        }
        let mut combo = BmbpCombo::new();
        combo.set_value(item.get_dict_value().clone());
        combo.set_label(item.get_dict_name().clone());
        combo.set_children(child_combo);
        bmbp_combo.push(combo)
    }
    bmbp_combo
}

