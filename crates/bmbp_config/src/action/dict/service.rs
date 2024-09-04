use bmbp_app_util::{parse_orm, parse_user_orm};
use bmbp_http_type::{BmbpPageReq, BmbpResp, BmbpRespErr, PageData};
use bmbp_rdbc_orm::RdbcOrm;
use bmbp_rdbc_sql::{InsertWrapper, QueryWrapper, RdbcColumn, RdbcConcatType, RdbcTableFilter, RdbcTableFilterImpl, RdbcTableWrapper, UpdateWrapper};
use bmbp_rdbc_type::RdbcIdent;
use bmbp_rdbc_type::RdbcTable;
use bmbp_util::{BMBP_TREE_ROOT_NODE, BmbpId, BmbpTreeUtil, current_time};
use salvo::Depot;

use bmbp_curd::{BmbpCurdDao, BmbpCurdService};

use crate::action::dict::bean::{BatchComboVo, BatchReqVo, BmbpCombo, BmbpCombos, BmbpDict, BmbpDictColumn, BmbpDisplay};

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
        BmbpCurdDao::execute_query_page::<BmbpDict>(orm, Some(page_req.get_page_no().clone()), Some(page_req.get_page_size().clone()), &query_wrapper).await
    }
    pub(crate) async fn find_dict_list(depot: &mut Depot, params: &BmbpDict) -> BmbpResp<Option<Vec<BmbpDict>>> {
        let query_wrapper = Self::build_dict_query_wrapper(depot, Some(params)).await?;
        let orm = parse_orm(depot)?;
        BmbpCurdDao::execute_query_list::<BmbpDict>(orm, &query_wrapper).await
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
        let dict_vec = BmbpCurdDao::execute_query_list::<BmbpDict>(orm, &query_wrapper).await?;
        if dict_vec.is_some() {
            Ok(Some(BmbpTreeUtil::build_tree::<BmbpDict>(dict_vec.unwrap())))
        } else {
            Ok(None)
        }
    }


    pub(crate) async fn find_dict_info(depot: &mut Depot, params: Option<&String>) -> BmbpResp<Option<BmbpDict>> {
        let orm = parse_orm(depot)?;
        BmbpCurdService::find_info_by_id::<BmbpDict>(orm, params).await
    }
    pub(crate) async fn find_dict_info_by_alias(depot: &mut Depot, alias: Option<&String>) -> BmbpResp<Option<BmbpDict>> {
        if alias.is_none() || alias.as_ref().unwrap().is_empty() {
            return Err(BmbpRespErr::err(Some("VALID".to_string()), Some("请传入字典编码".to_string())));
        }
        let mut query_wrapper = QueryWrapper::new_from::<BmbpDict>();
        query_wrapper.eq_(BmbpDictColumn::DictAlias, alias.unwrap().clone());
        let orm = parse_orm(depot)?;
        BmbpCurdDao::execute_query_one::<BmbpDict>(orm, &query_wrapper).await
    }
    pub(crate) async fn find_dict_info_by_code(depot: &mut Depot, code: Option<&String>) -> BmbpResp<Option<BmbpDict>> {
        if code.is_none() || code.as_ref().unwrap().is_empty() {
            return Ok(None);
        }
        let mut query_wrapper = QueryWrapper::new_from::<BmbpDict>();
        query_wrapper.eq_(BmbpDictColumn::DictCode, code.unwrap().clone());
        let orm = parse_orm(depot)?;
        BmbpCurdDao::execute_query_one::<BmbpDict>(orm, &query_wrapper).await
    }
    pub(crate) async fn find_dict_info_in_alias(depot: &mut Depot, alias: &[String]) -> BmbpResp<Option<Vec<BmbpDict>>> {
        if alias.is_empty() {
            return Err(BmbpRespErr::err(Some("VALID".to_string()), Some("请传入字典编码".to_string())));
        }
        let mut query_wrapper = QueryWrapper::new_from::<BmbpDict>();
        query_wrapper.in_v_slice(BmbpDictColumn::DictAlias, alias);
        let orm = parse_orm(depot)?;
        BmbpCurdDao::execute_query_list::<BmbpDict>(orm, &query_wrapper).await
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

        if params.get_dict_alias().as_ref().is_none() || params.get_dict_alias().as_ref().unwrap().is_empty() {
            return Err(BmbpRespErr::err(Some("VALID".to_string()), Some("请传入字典别名".to_string())));
        }
        if params.get_dict_name().as_ref().is_none() || params.get_dict_name().as_ref().unwrap().is_empty() {
            return Err(BmbpRespErr::err(Some("VALID".to_string()), Some("请传入字典名称".to_string())));
        }
        if params.get_dict_value().as_ref().is_none() || params.get_dict_value().as_ref().unwrap().is_empty() {
            return Err(BmbpRespErr::err(Some("VALID".to_string()), Some("请传入字典值".to_string())));
        }
        if params.get_dict_parent_code().as_ref().is_none() || params.get_dict_parent_code().as_ref().unwrap().is_empty() {
            params.set_dict_parent_code(Some(BMBP_TREE_ROOT_NODE.to_string()));
        }

        let dict_code = BmbpId::simple_uuid();
        params.set_dict_code(Some(dict_code.clone()));
        let dict_name = params.get_dict_name().clone().unwrap_or("".to_string());
        if let Some(parent_node) = Self::find_dict_info_by_code(depot, params.get_dict_parent_code().as_ref()).await? {
            let parent_code_path = parent_node.get_dict_code_path().clone().unwrap_or("".to_string());
            let parent_name_path = parent_node.get_dict_name_path().clone().unwrap_or("".to_string());
            if parent_code_path.is_empty() || parent_name_path.is_empty() {
                return Err(BmbpRespErr::err(Some("VALID".to_string()), Some("父级节点信息异常,请联系管理员".to_string())));
            }
            params.set_dict_code_path(Some(format!("{}{},", parent_code_path, dict_code)));
            params.set_dict_name_path(Some(format!("{}{},", parent_name_path, dict_name)));
        } else {
            params.set_dict_code_path(Some(format!("{},{},", params.get_dict_parent_code().as_ref().unwrap(), dict_code)));
            params.set_dict_name_path(Some(format!("{},{},", params.get_dict_parent_code().as_ref().unwrap(), dict_name)));
        }
        // tree_grade;
        let tree_grade = params.get_dict_code_path().as_ref().unwrap().split(",").count() - 2;
        params.set_dict_tree_grade(Some(tree_grade as u32));
        let (user, orm) = parse_user_orm(depot);
        // 校验别名是否重复
        Self::check_save_alias(orm.unwrap(), params.get_dict_alias().clone().as_ref().unwrap(), params.get_data_id().clone()).await?;
        Self::check_save_name(orm.unwrap(), params.get_dict_parent_code().clone().unwrap(), params.get_dict_name().clone().unwrap(), params.get_data_id().clone()).await?;
        Self::check_save_value(orm.unwrap(), params.get_dict_parent_code().clone().unwrap(), params.get_dict_value().clone().unwrap(), params.get_data_id().clone()).await?;

        let mut insert_wrapper = InsertWrapper::new();
        insert_wrapper.table(BmbpDict::get_table().get_ident());

        insert_wrapper.insert_column_value(BmbpDictColumn::DictCode.get_ident(), params.get_dict_code().as_ref().unwrap());
        insert_wrapper.insert_column_value(BmbpDictColumn::DictParentCode.get_ident(), params.get_dict_parent_code().as_ref().unwrap());
        insert_wrapper.insert_column_value(BmbpDictColumn::DictName.get_ident(), params.get_dict_name().as_ref().unwrap());
        insert_wrapper.insert_column_value(BmbpDictColumn::DictCodePath.get_ident(), params.get_dict_code_path().as_ref().unwrap());
        insert_wrapper.insert_column_value(BmbpDictColumn::DictNamePath.get_ident(), params.get_dict_name_path().as_ref().unwrap());
        insert_wrapper.insert_column_value(BmbpDictColumn::DictTreeGrade.get_ident(), params.get_dict_tree_grade().unwrap_or(1));
        insert_wrapper.insert_column_value(BmbpDictColumn::DictAlias.get_ident(), params.get_dict_alias().as_ref().unwrap());
        insert_wrapper.insert_column_value(BmbpDictColumn::DictValue.get_ident(), params.get_dict_value().as_ref().unwrap());

        insert_wrapper.insert_column_value(BmbpDictColumn::DataId.get_ident(), params.get_data_id().as_ref().unwrap());
        insert_wrapper.insert_column_value(BmbpDictColumn::DataLevel.get_ident(), params.get_data_level().clone().unwrap_or("0".to_string()));
        insert_wrapper.insert_column_value(BmbpDictColumn::DataFlag.get_ident(), params.get_data_flag().clone().unwrap_or("0".to_string()));
        insert_wrapper.insert_column_value(BmbpDictColumn::DataSort.get_ident(), params.get_data_sort().unwrap_or(0));
        insert_wrapper.insert_column_value(BmbpDictColumn::DataStatus.get_ident(), params.get_data_status().clone().unwrap_or("0".to_string()));
        insert_wrapper.insert_column_value(BmbpDictColumn::DataCreateTime.get_ident(), current_time());
        insert_wrapper.insert_column_value(BmbpDictColumn::DataUpdateTime.get_ident(), current_time());
        let current_user = match user {
            Some(u) => {
                u.get_id().clone().unwrap_or("".to_string())
            }
            None => {
                "".to_string()
            }
        };
        insert_wrapper.insert_column_value(BmbpDictColumn::DataCreateUser.get_ident(), current_user.clone());
        insert_wrapper.insert_column_value(BmbpDictColumn::DataUpdateUser.get_ident(), current_user.clone());
        insert_wrapper.insert_column_value(BmbpDictColumn::DataOwnerOrg.get_ident(), "");
        insert_wrapper.insert_column_value(BmbpDictColumn::DataSign.get_ident(), "");

        BmbpCurdDao::execute_insert::<BmbpDict>(orm.unwrap(), &insert_wrapper).await?;
        Self::find_dict_info(depot, params.get_data_id().as_ref()).await
    }


    pub(crate) async fn update_dict(depot: &mut Depot, params: &mut BmbpDict) -> BmbpResp<Option<BmbpDict>> {
        if params.get_data_id().is_none() {
            return Err(BmbpRespErr::err(Some("VALID".to_string()), Some("请传入字典标识".to_string())));
        }
        let mut old_dict_name = "".to_string();
        let mut old_dict_parent_code = "".to_string();
        let mut old_dict_code_path = "".to_string();
        let mut old_dict_name_path = "".to_string();
        if let Some(mut dict_info) = Self::find_dict_info(depot, params.get_data_id().as_ref()).await? {
            old_dict_parent_code = dict_info.get_dict_parent_code().clone().unwrap();
            old_dict_name = dict_info.get_dict_name().clone().unwrap();
            old_dict_code_path = dict_info.get_dict_code_path().clone().unwrap();
            old_dict_name_path = dict_info.get_dict_name_path().clone().unwrap();
            if params.get_dict_code().is_none() {
                params.set_dict_code(dict_info.get_dict_code().clone());
            }
            if params.get_dict_parent_code().is_none() {
                params.set_dict_parent_code(dict_info.get_dict_parent_code().clone());
            }
            if params.get_dict_name().is_none() {
                params.set_dict_name(dict_info.get_dict_name().clone());
            }
            if params.get_dict_alias().is_none() {
                params.set_dict_alias(dict_info.get_dict_alias().clone());
            }
            if params.get_dict_value().is_none() {
                params.set_dict_value(dict_info.get_dict_value().clone());
            }
            if params.get_data_sort().is_none() {
                params.set_data_sort(dict_info.get_data_sort().clone());
            }
        } else {
            return Err(BmbpRespErr::err(Some("REQUEST".to_string()), Some("未找到字典信息".to_string())));
        }

        let mut dict_code_path = "".to_string();
        let mut dict_name_path = "".to_string();
        if let Some(parent_node) = Self::find_dict_info_by_code(depot, params.get_dict_parent_code().as_ref()).await? {
            dict_code_path = format!("{},{},", parent_node.get_dict_code_path().clone().unwrap(), parent_node.get_dict_name_path().clone().unwrap());
            dict_name_path = format!("{},{},", parent_node.get_dict_name_path().clone().unwrap(), params.get_dict_name().as_ref().unwrap());
        } else {
            if params.get_dict_parent_code().as_ref().unwrap() == BMBP_TREE_ROOT_NODE {
                dict_code_path = format!("{},{},", BMBP_TREE_ROOT_NODE.to_string(), params.get_dict_code().as_ref().unwrap());
                dict_name_path = format!("{},{},", BMBP_TREE_ROOT_NODE.to_string(), params.get_dict_name().as_ref().unwrap());
            } else {
                return Err(BmbpRespErr::err(Some("REQUEST".to_string()), Some("未找到上级字典信息".to_string())));
            }
        }
        let tree_grade = params.get_dict_code_path().as_ref().unwrap().split(",").count() - 2;
        params.set_dict_tree_grade(Some(tree_grade as u32));

        // 校验别名是否重复
        let orm = parse_orm(depot)?;
        Self::check_save_alias(orm, params.get_dict_alias().clone().as_ref().unwrap(), params.get_data_id().clone()).await?;
        Self::check_save_name(orm, params.get_dict_parent_code().clone().unwrap(), params.get_dict_name().clone().unwrap(), params.get_data_id().clone()).await?;
        Self::check_save_value(orm, params.get_dict_parent_code().clone().unwrap(), params.get_dict_value().clone().unwrap(), params.get_data_id().clone()).await?;


        let mut update_wrapper = UpdateWrapper::new();
        update_wrapper.table(BmbpDict::get_table().get_ident());
        update_wrapper.set(BmbpDictColumn::DictCode, params.get_dict_code().as_ref().unwrap());
        update_wrapper.set(BmbpDictColumn::DictParentCode, params.get_dict_parent_code().as_ref().unwrap());
        update_wrapper.set(BmbpDictColumn::DictName, params.get_dict_name().as_ref().unwrap());
        update_wrapper.set(BmbpDictColumn::DictAlias, params.get_dict_alias().as_ref().unwrap());
        update_wrapper.set(BmbpDictColumn::DictValue, params.get_dict_value().as_ref().unwrap());
        update_wrapper.set(BmbpDictColumn::DictCodePath, &dict_code_path);
        update_wrapper.set(BmbpDictColumn::DictNamePath, &dict_name_path);
        update_wrapper.set(BmbpDictColumn::DictTreeGrade, params.get_dict_tree_grade().unwrap());
        update_wrapper.set(BmbpDictColumn::DataSort, params.get_data_sort().unwrap());
        update_wrapper.set(BmbpDictColumn::DataUpdateTime, current_time());
        update_wrapper.set(BmbpDictColumn::DataUpdateUser, "");
        update_wrapper.eq_(BmbpDictColumn::DataId, params.get_data_id().as_ref().unwrap());

        BmbpCurdDao::execute_update::<BmbpDict>(orm, &update_wrapper).await?;
        if &old_dict_name != params.get_dict_name().as_ref().unwrap() || &old_dict_parent_code != params.get_dict_parent_code().as_ref().unwrap() {
            Self::update_children_dict_path(orm, &old_dict_code_path, &dict_code_path, &old_dict_name_path, &dict_name_path).await?;
        }
        Self::find_dict_info(depot, params.get_data_id().as_ref()).await
    }

    pub(crate) async fn enable_dict(depot: &mut Depot, params: Option<&String>) -> BmbpResp<Option<u64>> {
        let orm = parse_orm(depot)?;
        BmbpCurdService::enable::<BmbpDict>(orm, params).await
    }

    pub(crate) async fn batch_enable_dict(depot: &mut Depot, params: &BatchReqVo) -> BmbpResp<Option<u64>> {
        let orm = parse_orm(depot)?;
        let data_ids = params.get_ids().clone().unwrap_or(vec![]);
        BmbpCurdService::batch_enable::<BmbpDict>(orm, data_ids.as_slice()).await
    }

    pub(crate) async fn disable_dict(depot: &mut Depot, params: Option<&String>) -> BmbpResp<Option<u64>> {
        let orm = parse_orm(depot)?;
        BmbpCurdService::disable::<BmbpDict>(orm, params).await
    }

    pub(crate) async fn batch_disable_dict(depot: &mut Depot, params: &BatchReqVo) -> BmbpResp<Option<u64>> {
        let orm = parse_orm(depot)?;
        let data_ids = params.get_ids().clone().unwrap_or(vec![]);
        BmbpCurdService::batch_disable::<BmbpDict>(orm, data_ids.as_slice()).await
    }

    pub(crate) async fn remove_dict(depot: &mut Depot, params: Option<&String>) -> BmbpResp<Option<u64>> {
        let orm = parse_orm(depot)?;
        BmbpCurdService::remove::<BmbpDict>(orm, params).await
    }

    pub(crate) async fn batch_remove_dict(depot: &mut Depot, params: &BatchReqVo) -> BmbpResp<Option<u64>> {
        let orm = parse_orm(depot)?;
        let data_ids = params.get_ids().clone().unwrap_or(vec![]);
        BmbpCurdService::batch_remove::<BmbpDict>(orm, data_ids.as_slice()).await
    }

    pub(crate) async fn update_order(depot: &mut Depot, params: &BmbpDict) -> BmbpResp<Option<u64>> {
        let orm = parse_orm(depot)?;
        let data_id = params.get_data_id().as_ref();
        let order = params.get_data_sort().as_ref();
        BmbpCurdService::update_order::<BmbpDict>(orm, data_id, order).await
    }

    pub(crate) async fn update_parent(depot: &mut Depot, params: &mut BmbpDict) -> BmbpResp<Option<u64>> {
        if params.get_dict_parent_code().is_none() || params.get_dict_parent_code().as_ref().unwrap().is_empty() {
            return Err(BmbpRespErr::err(Some("REQUEST".to_string()), Some("字典父级不能为空".to_string())));
        }
        if params.get_data_id().is_none() || params.get_data_id().as_ref().unwrap().is_empty() {
            return Err(BmbpRespErr::err(Some("REQUEST".to_string()), Some("字典标识不能为空".to_string())));
        }
        Self::update_dict(depot, params).await?;
        Ok(Some(1))
    }

    pub(crate) async fn find_dict_combo(depot: &mut Depot, alias: Option<&String>, cascade: Option<&String>) -> BmbpResp<Option<Vec<BmbpCombo>>> {
        let mut batch_req = BatchComboVo::new();
        if let Some(a) = alias {
            batch_req.set_codes(Some(vec![a.clone()]));
        }
        if let Some(ca) = cascade {
            batch_req.set_cascade(Some(ca.clone()));
        }
        let dict_combo = Self::find_dict_combos(depot, &batch_req).await?;
        match dict_combo {
            Some(v) => Ok(Some(v.get(&alias.unwrap().clone()).unwrap_or(&vec![]).clone())),
            None => Ok(None),
        }
    }

    pub(crate) async fn find_dict_combos(depot: &mut Depot, params: &BatchComboVo) -> BmbpResp<Option<BmbpCombos>> {
        let code = params.get_codes().as_ref().unwrap_or(&vec![]).clone();
        let cascade = params.get_cascade().as_ref().clone();
        let dict_vec = Self::find_dict_children_by_alias(depot, code.as_slice(), cascade).await?;
        Self::convert_dict_to_combos(dict_vec)
    }

    pub(crate) async fn find_dict_display(depot: &mut Depot, alias: Option<&String>, cascade: Option<&String>) -> BmbpResp<Option<BmbpDisplay>> {
        let dict_vec = Self::find_dict_combo(depot, alias, cascade).await?;
        let mut display = BmbpDisplay::new();
        if let Some(v) = dict_vec {
            for item in v.as_slice() {
                display.insert(item.get_value().as_ref().unwrap_or(&"".to_string()).to_string(), item.get_label().as_ref().unwrap_or(&"".to_string()).to_string());
            }
        }
        Ok(Some(display))
    }

    pub(crate) async fn find_dict_displays(depot: &mut Depot, params: &BatchComboVo) -> BmbpResp<Option<BmbpDisplay>> {
        let dict_vec = Self::find_dict_combos(depot, &params).await?;
        Self::convert_combos_display(dict_vec)
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
    async fn find_dict_children_by_alias(depot: &mut Depot, alias: &[String], cascade: Option<&String>) -> BmbpResp<Vec<BmbpDict>> {
        let dict_vec_op = Self::find_dict_info_in_alias(depot, alias).await?;
        if dict_vec_op.is_none() || dict_vec_op.as_ref().unwrap().is_empty() {
            return Err(BmbpRespErr::err(Some("REQUEST".to_string()), Some("未找到字典信息".to_string())));
        }
        let mut dict_code_vec = vec![];
        for item in dict_vec_op.as_ref().unwrap() {
            dict_code_vec.push(item.get_dict_code().as_ref().unwrap().clone());
        }
        let mut query_wrapper = QueryWrapper::new_from::<BmbpDict>();
        let mut filter = RdbcTableFilterImpl::concat(RdbcConcatType::Or);
        for code in dict_code_vec.as_slice() {
            filter.like_value(BmbpDictColumn::DictCodePath, code.clone());
        }
        query_wrapper.add_filter(filter);
        if cascade.is_none() || (!cascade.as_ref().unwrap().as_str().eq("1") && !cascade.as_ref().unwrap().as_str().eq("true")) {
            let mut filter2 = RdbcTableFilterImpl::concat(RdbcConcatType::Or);
            filter2.in_v_slice(BmbpDictColumn::DictCode.get_ident(), dict_code_vec.as_slice());
            filter2.in_v_slice(BmbpDictColumn::DictParentCode, dict_code_vec.as_slice());
            query_wrapper.add_filter(filter2);
        }
        let orm = parse_orm(depot)?;
        let dict_vec = BmbpCurdDao::execute_query_list::<BmbpDict>(orm, &query_wrapper).await?;
        Ok(BmbpTreeUtil::build_tree::<BmbpDict>(dict_vec.unwrap_or(vec![])))
    }
    fn convert_dict_to_combos(dict_vec: Vec<BmbpDict>) -> BmbpResp<Option<BmbpCombos>> {
        let mut combos = BmbpCombos::new();
        for dict in dict_vec.as_slice() {
            let code = dict.get_dict_alias().as_ref().unwrap();
            let combo_vec = Self::convert_to_vec(dict.get_dict_children().as_ref().unwrap().as_slice());
            combos.insert(code.to_string(), combo_vec);
        }
        Ok(Some(combos))
    }
    fn convert_combos_display(dict_vec: Option<BmbpCombos>) -> BmbpResp<Option<BmbpDisplay>> {
        let mut displays = BmbpDisplay::new();
        if dict_vec.is_none() {
            return Ok(Some(displays));
        }

        for (k, v) in dict_vec.as_ref().unwrap() {
            let child_display = Self::convert_child_combo_to_display(k, v);
            displays.extend(child_display);
        }

        Ok(Some(displays))
    }
    async fn check_save_alias(orm: &RdbcOrm, dict_alias: &String, data_id: Option<String>) -> BmbpResp<()> {
        let mut query = QueryWrapper::new_from::<BmbpDict>();
        query.eq_(BmbpDictColumn::DictAlias, dict_alias.clone());
        query.ne_(BmbpDictColumn::DataId, data_id.clone());
        return match orm.select_one_by_query::<BmbpDict>(&query).await {
            Ok(dict) => {
                if dict.is_some() {
                    Err(BmbpRespErr::err(Some("REQUEST".to_string()), Some("字典别名已存在".to_string())))
                } else {
                    Ok(())
                }
            }
            Err(err) => {
                Err(BmbpRespErr::err(Some("DB".to_string()), Some(err.get_msg())))
            }
        };
    }
    async fn check_save_name(orm: &RdbcOrm, parent_code: String, dict_name: String, data_id: Option<String>) -> BmbpResp<()> {
        let mut query = QueryWrapper::new_from::<BmbpDict>();
        query.eq_(BmbpDictColumn::DictName, dict_name.clone());
        query.eq_(BmbpDictColumn::DictParentCode, parent_code.clone());
        query.ne_(BmbpDictColumn::DataId, data_id.clone());
        return match orm.select_one_by_query::<BmbpDict>(&query).await {
            Ok(dict) => {
                if dict.is_some() {
                    Err(BmbpRespErr::err(Some("REQUEST".to_string()), Some("字典名称已存在".to_string())))
                } else {
                    Ok(())
                }
            }
            Err(err) => {
                Err(BmbpRespErr::err(Some("DB".to_string()), Some(err.get_msg())))
            }
        };
    }
    async fn check_save_value(orm: &RdbcOrm, parent_code: String, dict_value: String, data_id: Option<String>) -> BmbpResp<()> {
        let mut query = QueryWrapper::new_from::<BmbpDict>();
        query.eq_(BmbpDictColumn::DictValue, dict_value.clone());
        query.eq_(BmbpDictColumn::DictParentCode, parent_code.clone());
        query.ne_(BmbpDictColumn::DataId, data_id.clone());
        return match orm.select_one_by_query::<BmbpDict>(&query).await {
            Ok(dict) => {
                if dict.is_some() {
                    Err(BmbpRespErr::err(Some("VALID".to_string()), Some("字典值已存在".to_string())))
                } else {
                    Ok(())
                }
            }
            Err(err) => {
                Err(BmbpRespErr::err(Some("DB".to_string()), Some(err.get_msg())))
            }
        };
    }
    async fn update_children_dict_path(orm: &RdbcOrm, old_code_path: &String, new_code_path: &String, old_name_path: &String, new_name_path: &String) -> BmbpResp<Option<u64>> {
        let mut update = UpdateWrapper::new();
        update.table(BmbpDict::get_table().get_ident())
            .set(BmbpDictColumn::DictNamePath, RdbcColumn::replace(BmbpDictColumn::DictNamePath.get_ident(), old_name_path, new_name_path))
            .set(BmbpDictColumn::DictCodePath, RdbcColumn::replace(BmbpDictColumn::DictCodePath.get_ident(), old_code_path, new_code_path));
        update.like_left_value(BmbpDictColumn::DictCodePath, old_code_path);
        BmbpCurdDao::execute_update::<BmbpDict>(orm, &update).await
    }
    fn convert_child_combo_to_display(code: &String, dict_vec: &Vec<BmbpCombo>) -> BmbpDisplay {
        let mut displays = BmbpDisplay::new();
        for item in dict_vec {
            let key = format!("{}.{}", code, item.get_value().as_ref().unwrap_or(&"".to_string()));
            displays.insert(key.clone(), item.get_label().as_ref().unwrap_or(&"-".to_string()).to_string());
            let children = item.get_children();
            let child_display = Self::convert_child_combo_to_display(&key, children);
            displays.extend(child_display);
        }
        displays
    }
    fn convert_to_vec(dict_vec: &[BmbpDict]) -> Vec<BmbpCombo> {
        let mut bmbp_combo = vec![];
        for item in dict_vec {
            let mut child_combo = vec![];
            if item.get_dict_children().is_some() && !item.get_dict_children().as_ref().unwrap().is_empty() {
                child_combo = Self::convert_to_vec(item.get_dict_children().as_ref().unwrap().as_slice());
            }
            let mut combo = BmbpCombo::new();
            combo.set_value(item.get_dict_value().clone());
            combo.set_label(item.get_dict_name().clone());
            combo.set_children(child_combo);
            bmbp_combo.push(combo)
        }
        bmbp_combo
    }

}


