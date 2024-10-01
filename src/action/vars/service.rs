use bmbp_app_util::{parse_orm, parse_user_orm};
use bmbp_http_type::{BmbpPageReq, BmbpResp, BmbpRespErr, PageData};

use bmbp_rdbc::DeleteWrapper;
use bmbp_rdbc::InsertWrapper;
use bmbp_rdbc::QueryWrapper;
use bmbp_rdbc::RdbcColumn;
use bmbp_rdbc::RdbcIdent;
use bmbp_rdbc::RdbcOrm;
use bmbp_rdbc::RdbcTable;
use bmbp_rdbc::RdbcTableFilter;
use bmbp_rdbc::RdbcTableWrapper;
use bmbp_rdbc::UpdateWrapper;
use bmbp_util::{current_time, BmbpId, BmbpTreeUtil, BMBP_TREE_ROOT_NODE};
use salvo::Depot;

use bmbp_curd::{BmbpCurdDao, BmbpCurdService};

use super::bean::BatchReqVo;
use super::bean::BmbpVars;
use super::bean::BmbpVarsColumn;

pub struct BmbpVarsService;

impl BmbpVarsService {
    pub(crate) async fn vars_find_tree(
        depot: &mut Depot,
        params: &BmbpVars,
    ) -> BmbpResp<Option<Vec<BmbpVars>>> {
        let vars_vec = Self::vars_find_list(depot, params).await?;
        if let Some(dic) = vars_vec {
            Ok(Some(BmbpTreeUtil::build_tree::<BmbpVars>(dic)))
        } else {
            Ok(None)
        }
    }
    pub(crate) async fn vars_find_page(
        depot: &mut Depot,
        page_req: &BmbpPageReq<BmbpVars>,
    ) -> BmbpResp<Option<PageData<BmbpVars>>> {
        let query_wrapper = Self::build_vars_query_wrapper(depot, page_req.get_params()).await?;
        let orm = parse_orm(depot)?;
        BmbpCurdDao::execute_query_page::<BmbpVars>(
            orm,
            Some(page_req.get_page_no().clone()),
            Some(page_req.get_page_size().clone()),
            &query_wrapper,
        )
        .await
    }
    pub(crate) async fn vars_find_list(
        depot: &mut Depot,
        params: &BmbpVars,
    ) -> BmbpResp<Option<Vec<BmbpVars>>> {
        let query_wrapper = Self::build_vars_query_wrapper(depot, Some(params)).await?;
        let orm = parse_orm(depot)?;
        BmbpCurdDao::execute_query_list::<BmbpVars>(orm, &query_wrapper).await
    }
    pub(crate) async fn vars_find_tree_ignore(
        depot: &mut Depot,
        params: &BmbpVars,
    ) -> BmbpResp<Option<Vec<BmbpVars>>> {
        let mut query_wrapper = QueryWrapper::new_from::<BmbpVars>();
        if let Some(vars_id) = params.get_data_id() {
            let vars = Self::vars_find_info(depot, Some(vars_id)).await?;
            if vars.is_none() {
                return Err(BmbpRespErr::err(
                    Some("DB".to_string()),
                    Some("未找到参数信息".to_string()),
                ));
            }
            query_wrapper.not_like_left_(
                BmbpVarsColumn::VarsCodePath,
                vars.unwrap().get_vars_code_path(),
            );
        }
        if let Some(vars_code) = params.get_vars_code() {
            query_wrapper.not_like_left_(BmbpVarsColumn::VarsCodePath, vars_code.clone());
        }
        if let Some(vars_parent_code) = params.get_vars_parent_code() {
            query_wrapper.not_like_left_(BmbpVarsColumn::VarsCodePath, vars_parent_code.clone());
        }
        let orm = parse_orm(depot)?;
        let vars_vec = BmbpCurdDao::execute_query_list::<BmbpVars>(orm, &query_wrapper).await?;
        if vars_vec.is_some() {
            Ok(Some(BmbpTreeUtil::build_tree::<BmbpVars>(
                vars_vec.unwrap(),
            )))
        } else {
            Ok(None)
        }
    }

    pub(crate) async fn vars_find_info(
        depot: &mut Depot,
        params: Option<&String>,
    ) -> BmbpResp<Option<BmbpVars>> {
        let orm = parse_orm(depot)?;
        BmbpCurdService::find_info_by_id::<BmbpVars>(orm, params).await
    }

    pub(crate) async fn vars_find_info_by_code(
        depot: &mut Depot,
        code: Option<&String>,
    ) -> BmbpResp<Option<BmbpVars>> {
        if code.is_none() || code.as_ref().unwrap().is_empty() {
            return Ok(None);
        }
        let mut query_wrapper = QueryWrapper::new_from::<BmbpVars>();
        query_wrapper.eq_(BmbpVarsColumn::VarsCode, code.unwrap().clone());
        let orm = parse_orm(depot)?;
        BmbpCurdDao::execute_query_one::<BmbpVars>(orm, &query_wrapper).await
    }

    pub(crate) async fn vars_save(
        depot: &mut Depot,
        params: &mut BmbpVars,
    ) -> BmbpResp<Option<BmbpVars>> {
        let vars_info = Self::vars_find_info(depot, params.get_data_id().as_ref()).await?;
        if vars_info.is_none() {
            Self::vars_insert(depot, params).await
        } else {
            Self::vars_update(depot, params).await
        }
    }

    pub(crate) async fn vars_insert(
        depot: &mut Depot,
        params: &mut BmbpVars,
    ) -> BmbpResp<Option<BmbpVars>> {
        if params.get_data_id().is_none() {
            params.set_data_id(Some(BmbpId::simple_uuid()));
        }

        if params.get_vars_code().as_ref().is_none()
            || params.get_vars_code().as_ref().unwrap().is_empty()
        {
            return Err(BmbpRespErr::err(
                Some("VALID".to_string()),
                Some("请传入参数编码".to_string()),
            ));
        }

        if params.get_vars_name().as_ref().is_none()
            || params.get_vars_name().as_ref().unwrap().is_empty()
        {
            return Err(BmbpRespErr::err(
                Some("VALID".to_string()),
                Some("请传入参数名称".to_string()),
            ));
        }

        if params.get_vars_parent_code().as_ref().is_none()
            || params.get_vars_parent_code().as_ref().unwrap().is_empty()
        {
            params.set_vars_parent_code(Some(BMBP_TREE_ROOT_NODE.to_string()));
        }

        let vars_name = params.get_vars_name().clone().unwrap_or("".to_string());
        let vars_code = params.get_vars_code().clone().unwrap_or("".to_string());
        if let Some(parent_node) =
            Self::vars_find_info_by_code(depot, params.get_vars_parent_code().as_ref()).await?
        {
            let parent_code_path = parent_node
                .get_vars_code_path()
                .clone()
                .unwrap_or("".to_string());
            let parent_name_path = parent_node
                .get_vars_name_path()
                .clone()
                .unwrap_or("".to_string());
            if parent_code_path.is_empty() || parent_name_path.is_empty() {
                return Err(BmbpRespErr::err(
                    Some("VALID".to_string()),
                    Some("父级节点信息异常,请联系管理员".to_string()),
                ));
            }
            params.set_vars_code_path(Some(format!("{}{}.", parent_code_path, vars_code)));
            params.set_vars_name_path(Some(format!("{}{}.", parent_name_path, vars_name)));
        } else {
            params.set_vars_code_path(Some(format!(
                "{}.{}.",
                params.get_vars_parent_code().as_ref().unwrap(),
                vars_code
            )));
            params.set_vars_name_path(Some(format!(
                "{}.{}.",
                params.get_vars_parent_code().as_ref().unwrap(),
                vars_name
            )));
        }
        // tree_grade;
        let tree_grade = params
            .get_vars_code_path()
            .as_ref()
            .unwrap()
            .split(".")
            .count()
            - 2;
        params.set_vars_tree_grade(Some(tree_grade as u32));
        let (user, orm) = parse_user_orm(depot);
        // 校验编码是否重复
        Self::check_same_code(
            orm.unwrap(),
            params.get_vars_parent_code().clone().unwrap(),
            params.get_vars_code().clone().unwrap(),
            params.get_data_id().clone(),
        )
        .await?;
        Self::check_same_name(
            orm.unwrap(),
            params.get_vars_parent_code().clone().unwrap(),
            params.get_vars_name().clone().unwrap(),
            params.get_data_id().clone(),
        )
        .await?;
        let mut insert_wrapper = InsertWrapper::new();
        insert_wrapper.table(BmbpVars::get_table());

        insert_wrapper.insert_column_value(
            BmbpVarsColumn::VarsCode.get_ident(),
            params.get_vars_code().as_ref().unwrap(),
        );
        insert_wrapper.insert_column_value(
            BmbpVarsColumn::VarsParentCode.get_ident(),
            params.get_vars_parent_code().as_ref().unwrap(),
        );
        insert_wrapper.insert_column_value(
            BmbpVarsColumn::VarsName.get_ident(),
            params.get_vars_name().as_ref().unwrap(),
        );
        insert_wrapper.insert_column_value(
            BmbpVarsColumn::VarsCodePath.get_ident(),
            params.get_vars_code_path().as_ref().unwrap(),
        );
        insert_wrapper.insert_column_value(
            BmbpVarsColumn::VarsNamePath.get_ident(),
            params.get_vars_name_path().as_ref().unwrap(),
        );
        insert_wrapper.insert_column_value(
            BmbpVarsColumn::VarsTreeGrade.get_ident(),
            params.get_vars_tree_grade().unwrap_or(1),
        );
        insert_wrapper.insert_column_value(
            BmbpVarsColumn::VarsValue.get_ident(),
            params.get_vars_value().as_ref().unwrap_or(&"".to_string()),
        );

        insert_wrapper.insert_column_value(
            BmbpVarsColumn::DataId.get_ident(),
            params.get_data_id().as_ref().unwrap(),
        );
        insert_wrapper.insert_column_value(
            BmbpVarsColumn::DataLevel.get_ident(),
            params.get_data_level().clone().unwrap_or("0".to_string()),
        );
        insert_wrapper.insert_column_value(
            BmbpVarsColumn::DataFlag.get_ident(),
            params.get_data_flag().clone().unwrap_or("0".to_string()),
        );
        insert_wrapper.insert_column_value(
            BmbpVarsColumn::DataSort.get_ident(),
            params.get_data_sort().unwrap_or(0),
        );
        insert_wrapper.insert_column_value(
            BmbpVarsColumn::DataStatus.get_ident(),
            params.get_data_status().clone().unwrap_or("0".to_string()),
        );
        insert_wrapper.insert_column_value(
            BmbpVarsColumn::VarsOrder.get_ident(),
            params.get_vars_order().clone().unwrap_or(0usize),
        );

        insert_wrapper
            .insert_column_value(BmbpVarsColumn::DataCreateTime.get_ident(), current_time());
        insert_wrapper
            .insert_column_value(BmbpVarsColumn::DataUpdateTime.get_ident(), current_time());
        let current_user = match user {
            Some(u) => u.get_id().clone().unwrap_or("".to_string()),
            None => "".to_string(),
        };
        insert_wrapper.insert_column_value(
            BmbpVarsColumn::DataCreateUser.get_ident(),
            current_user.clone(),
        );
        insert_wrapper.insert_column_value(
            BmbpVarsColumn::DataUpdateUser.get_ident(),
            current_user.clone(),
        );
        insert_wrapper.insert_column_value(BmbpVarsColumn::DataOwnerOrg.get_ident(), "");
        insert_wrapper.insert_column_value(BmbpVarsColumn::DataSign.get_ident(), "");

        BmbpCurdDao::execute_insert::<BmbpVars>(orm.unwrap(), &insert_wrapper).await?;
        Self::vars_find_info(depot, params.get_data_id().as_ref()).await
    }

    pub(crate) async fn vars_update(
        depot: &mut Depot,
        params: &mut BmbpVars,
    ) -> BmbpResp<Option<BmbpVars>> {
        if params.get_data_id().is_none() {
            return Err(BmbpRespErr::err(
                Some("VALID".to_string()),
                Some("请传入参数标识".to_string()),
            ));
        }

        let vars_info_op = Self::vars_find_info(depot, params.get_data_id().as_ref()).await?;
        if vars_info_op.is_none() {
            return Err(BmbpRespErr::err(
                Some("REQUEST".to_string()),
                Some("未找到参数信息".to_string()),
            ));
        }
        let vars_info = vars_info_op.unwrap();

        let old_vars_parent_code = vars_info.get_vars_parent_code().clone().unwrap();
        let old_vars_name = vars_info.get_vars_name().clone().unwrap();
        let old_vars_code_path = vars_info.get_vars_code_path().clone().unwrap();
        let old_vars_name_path = vars_info.get_vars_name_path().clone().unwrap();
        if params.get_vars_code().is_none() {
            params.set_vars_code(vars_info.get_vars_code().clone());
        }
        if params.get_vars_parent_code().is_none() {
            params.set_vars_parent_code(vars_info.get_vars_parent_code().clone());
        }
        if params.get_vars_name().is_none() {
            params.set_vars_name(vars_info.get_vars_name().clone());
        }
        if params.get_vars_value().is_none() {
            params.set_vars_value(vars_info.get_vars_value().clone());
        }
        if params.get_data_sort().is_none() {
            params.set_data_sort(vars_info.get_data_sort().clone());
        }
        if params.get_vars_order().is_none() {
            params.set_vars_order(vars_info.get_vars_order().clone());
        }
        let (vars_code_path, vars_name_path) = if params.get_vars_parent_code().as_ref().unwrap()
            == BMBP_TREE_ROOT_NODE
        {
            (
                format!(
                    "{}.{}.",
                    BMBP_TREE_ROOT_NODE.to_string(),
                    params.get_vars_code().as_ref().unwrap()
                ),
                format!(
                    "{}.{}.",
                    BMBP_TREE_ROOT_NODE.to_string(),
                    params.get_vars_name().as_ref().unwrap()
                ),
            )
        } else {
            let parent_node_op =
                Self::vars_find_info_by_code(depot, params.get_vars_parent_code().as_ref()).await?;
            if parent_node_op.is_none() {
                return Err(BmbpRespErr::err(
                    Some("REQUEST".to_string()),
                    Some("未找到上级参数信息".to_string()),
                ));
            }

            let parent_node = parent_node_op.unwrap();
            (
                format!(
                    "{}{}.",
                    parent_node.get_vars_code_path().clone().unwrap(),
                    params.get_vars_code().clone().unwrap()
                ),
                format!(
                    "{}{}.",
                    parent_node.get_vars_name_path().clone().unwrap(),
                    params.get_vars_name().as_ref().unwrap()
                ),
            )
        };

        let tree_grade = &vars_code_path.split(".").count() - 2;
        params.set_vars_tree_grade(Some(tree_grade as u32));

        // 校验别名是否重复
        let orm = parse_orm(depot)?;
        // 校验编码是否重复
        Self::check_same_code(
            orm,
            params.get_vars_parent_code().clone().unwrap(),
            params.get_vars_code().clone().unwrap(),
            params.get_data_id().clone(),
        )
        .await?;
        Self::check_same_name(
            orm,
            params.get_vars_parent_code().clone().unwrap(),
            params.get_vars_name().clone().unwrap(),
            params.get_data_id().clone(),
        )
        .await?;
        let mut update_wrapper = UpdateWrapper::new();
        update_wrapper.table(BmbpVars::get_table());
        update_wrapper.set(
            BmbpVarsColumn::VarsCode,
            params.get_vars_code().as_ref().unwrap(),
        );
        update_wrapper.set(
            BmbpVarsColumn::VarsParentCode,
            params.get_vars_parent_code().as_ref().unwrap(),
        );
        update_wrapper.set(
            BmbpVarsColumn::VarsName,
            params.get_vars_name().as_ref().unwrap(),
        );
        update_wrapper.set(
            BmbpVarsColumn::VarsValue,
            params.get_vars_value().as_ref().unwrap_or(&"".to_string()),
        );
        update_wrapper.set(BmbpVarsColumn::VarsCodePath, &vars_code_path);
        update_wrapper.set(BmbpVarsColumn::VarsNamePath, &vars_name_path);
        update_wrapper.set(
            BmbpVarsColumn::VarsTreeGrade,
            params.get_vars_tree_grade().unwrap(),
        );
        update_wrapper.set(BmbpVarsColumn::DataSort, params.get_data_sort().unwrap());
        update_wrapper.set(BmbpVarsColumn::DataUpdateTime, current_time());
        update_wrapper.set(BmbpVarsColumn::DataUpdateUser, "");
        update_wrapper.set(BmbpVarsColumn::VarsOrder, params.get_vars_order().clone());

        update_wrapper.eq_(
            BmbpVarsColumn::DataId,
            params.get_data_id().as_ref().unwrap(),
        );

        BmbpCurdDao::execute_update::<BmbpVars>(orm, &update_wrapper).await?;
        if &old_vars_name != params.get_vars_name().as_ref().unwrap()
            || &old_vars_parent_code != params.get_vars_parent_code().as_ref().unwrap()
        {
            Self::update_children_vars_path(
                orm,
                &old_vars_code_path,
                &vars_code_path,
                &old_vars_name_path,
                &vars_name_path,
            )
            .await?;
        }
        Self::vars_find_info(depot, params.get_data_id().as_ref()).await
    }

    pub(crate) async fn vars_enable(
        depot: &mut Depot,
        params: Option<&String>,
    ) -> BmbpResp<Option<u64>> {
        let orm = parse_orm(depot)?;
        let vars_info: Option<BmbpVars> =
            BmbpCurdService::find_info_by_id::<BmbpVars>(orm, params).await?;
        if vars_info.is_none() {
            return Err(BmbpRespErr::err(
                Some("SERVICE".to_string()),
                Some("未找到参数信息".to_string()),
            ));
        }
        let code_path = vars_info
            .as_ref()
            .unwrap()
            .get_vars_code_path()
            .as_ref()
            .unwrap()
            .clone();
        let code_vec: Vec<&str> = code_path.split('.').filter(|&s| !s.is_empty()).collect();
        print!("==>{:#?}", code_vec);
        let mut update_wrapper = UpdateWrapper::new();
        update_wrapper.set("data_status", "1");
        update_wrapper.table(BmbpVars::get_table().get_ident());
        update_wrapper.in_v(BmbpVarsColumn::VarsCode, code_vec);
        BmbpCurdDao::execute_update::<BmbpVars>(orm, &update_wrapper).await
    }

    pub(crate) async fn vars_batch_enable(
        depot: &mut Depot,
        params: &BatchReqVo,
    ) -> BmbpResp<Option<u64>> {
        let mut u64 = 0;
        if let Some(ids) = params.get_ids() {
            for id in ids {
                let resp = Self::vars_enable(depot, Some(id)).await?;
                if resp.is_some() {
                    u64 += resp.unwrap();
                }
            }
        }
        Ok(Some(u64))
    }

    pub(crate) async fn vars_disable(
        depot: &mut Depot,
        params: Option<&String>,
    ) -> BmbpResp<Option<u64>> {
        let orm = parse_orm(depot)?;
        let vars_info: Option<BmbpVars> =
            BmbpCurdService::find_info_by_id::<BmbpVars>(orm, params).await?;
        if vars_info.is_none() {
            return Err(BmbpRespErr::err(
                Some("SERVICE".to_string()),
                Some("未找到参数信息".to_string()),
            ));
        }
        let code_path = vars_info
            .as_ref()
            .unwrap()
            .get_vars_code_path()
            .as_ref()
            .unwrap()
            .clone();
        let mut update_wrapper = UpdateWrapper::new();
        update_wrapper.set("data_status", "0");
        update_wrapper.table(BmbpVars::get_table());
        update_wrapper.like_left_(BmbpVarsColumn::VarsCodePath, code_path);
        BmbpCurdDao::execute_update::<BmbpVars>(orm, &update_wrapper).await
    }

    pub(crate) async fn vars_batch_disable(
        depot: &mut Depot,
        params: &BatchReqVo,
    ) -> BmbpResp<Option<u64>> {
        let mut u64 = 0;
        if let Some(ids) = params.get_ids() {
            for id in ids {
                let resp = Self::vars_disable(depot, Some(id)).await?;
                if resp.is_some() {
                    u64 += resp.unwrap();
                }
            }
        }
        Ok(Some(u64))
    }

    pub(crate) async fn vars_remove(
        depot: &mut Depot,
        params: Option<&String>,
    ) -> BmbpResp<Option<u64>> {
        let orm = parse_orm(depot)?;

        let vars_info = BmbpCurdService::find_info_by_id::<BmbpVars>(orm, params).await?;
        if vars_info.is_none() {
            return Err(BmbpRespErr::err(
                Some("SERVICE".to_string()),
                Some("未找到参数信息".to_string()),
            ));
        }
        let code_path = vars_info
            .as_ref()
            .unwrap()
            .get_vars_code_path()
            .as_ref()
            .unwrap()
            .clone();
        let mut delete_wrapper = DeleteWrapper::new();
        delete_wrapper.table(BmbpVars::get_table());
        delete_wrapper.like_left_(BmbpVarsColumn::VarsCodePath, code_path);
        let res = BmbpCurdDao::execute_delete::<BmbpVars>(orm, &delete_wrapper).await?;
        Ok(res)
    }

    pub(crate) async fn vars_batch_remove(
        depot: &mut Depot,
        params: &BatchReqVo,
    ) -> BmbpResp<Option<u64>> {
        let data_ids = params.get_ids().clone().unwrap_or(vec![]);
        let mut res = 0u64;
        for id in data_ids {
            let resp = Self::vars_remove(depot, Some(&id)).await?;
            if resp.is_some() {
                res += resp.unwrap();
            }
        }
        Ok(Some(res))
    }
    pub(crate) async fn vars_update_parent(
        depot: &mut Depot,
        params: &mut BmbpVars,
    ) -> BmbpResp<Option<u64>> {
        if params.get_vars_parent_code().is_none()
            || params.get_vars_parent_code().as_ref().unwrap().is_empty()
        {
            params.set_vars_parent_code(Some(BMBP_TREE_ROOT_NODE.to_string()));
        }
        if params.get_data_id().is_none() || params.get_data_id().as_ref().unwrap().is_empty() {
            return Err(BmbpRespErr::err(
                Some("REQUEST".to_string()),
                Some("参数标识不能为空".to_string()),
            ));
        }
        Self::vars_update(depot, params).await?;
        Ok(Some(1))
    }

    async fn build_vars_query_wrapper(
        depot: &mut Depot,
        params_op: Option<&BmbpVars>,
    ) -> BmbpResp<QueryWrapper> {
        let mut query_wrapper = QueryWrapper::new_from::<BmbpVars>();
        if let Some(params) = params_op {
            if let Some(vars_id) = params.get_data_id() {
                let vars = Self::vars_find_info(depot, Some(vars_id)).await?;
                if vars.is_none() {
                    return Err(BmbpRespErr::err(
                        Some("DB".to_string()),
                        Some("未找到参数信息".to_string()),
                    ));
                }
                query_wrapper.like_left_(BmbpVarsColumn::VarsCodePath, vars_id.clone());
            }
            if let Some(vars_code) = params.get_vars_code() {
                query_wrapper.like_left_(BmbpVarsColumn::VarsCodePath, vars_code.clone());
            }
            if let Some(vars_parent_code) = params.get_vars_parent_code() {
                query_wrapper.like_(BmbpVarsColumn::VarsCodePath, vars_parent_code.clone());
            }

            if let Some(vars_name) = params.get_vars_name() {
                query_wrapper.like_(BmbpVarsColumn::VarsName, vars_name.clone());
            }
            if let Some(data_status) = params.get_data_status() {
                query_wrapper.eq_(BmbpVarsColumn::DataStatus, data_status.clone());
            }
        }
        query_wrapper.order_by(BmbpVarsColumn::VarsTreeGrade, true);
        query_wrapper.order_by(BmbpVarsColumn::VarsParentCode, true);
        query_wrapper.order_by(BmbpVarsColumn::DataSort, true);
        query_wrapper.order_by(BmbpVarsColumn::DataCreateTime, true);
        Ok(query_wrapper)
    }

    async fn check_same_code(
        orm: &RdbcOrm,
        parent_code: String,
        vars_code: String,
        data_id: Option<String>,
    ) -> BmbpResp<()> {
        let mut query = QueryWrapper::new_from::<BmbpVars>();
        query.eq_(BmbpVarsColumn::VarsCode, vars_code.clone());
        query.eq_(BmbpVarsColumn::VarsParentCode, parent_code.clone());
        query.ne_(BmbpVarsColumn::DataId, data_id.clone());
        return match orm.select_one_by_query::<BmbpVars>(&query).await {
            Ok(vars) => {
                if vars.is_some() {
                    Err(BmbpRespErr::err(
                        Some("REQUEST".to_string()),
                        Some("参数编码已存在".to_string()),
                    ))
                } else {
                    Ok(())
                }
            }
            Err(err) => Err(BmbpRespErr::err(
                Some("DB".to_string()),
                Some(err.get_msg()),
            )),
        };
    }
    async fn check_same_name(
        orm: &RdbcOrm,
        parent_code: String,
        vars_name: String,
        data_id: Option<String>,
    ) -> BmbpResp<()> {
        let mut query = QueryWrapper::new_from::<BmbpVars>();
        query.eq_(BmbpVarsColumn::VarsName, vars_name.clone());
        query.eq_(BmbpVarsColumn::VarsParentCode, parent_code.clone());
        query.ne_(BmbpVarsColumn::DataId, data_id.clone());
        return match orm.select_one_by_query::<BmbpVars>(&query).await {
            Ok(vars) => {
                if vars.is_some() {
                    Err(BmbpRespErr::err(
                        Some("REQUEST".to_string()),
                        Some("参数名称已存在".to_string()),
                    ))
                } else {
                    Ok(())
                }
            }
            Err(err) => Err(BmbpRespErr::err(
                Some("DB".to_string()),
                Some(err.get_msg()),
            )),
        };
    }
    async fn update_children_vars_path(
        orm: &RdbcOrm,
        old_code_path: &String,
        new_code_path: &String,
        old_name_path: &String,
        new_name_path: &String,
    ) -> BmbpResp<Option<u64>> {
        let mut update = UpdateWrapper::new();
        update
            .table(BmbpVars::get_table().get_ident())
            .set(
                BmbpVarsColumn::VarsNamePath,
                RdbcColumn::replace(
                    BmbpVarsColumn::VarsNamePath.get_ident(),
                    old_name_path,
                    new_name_path,
                ),
            )
            .set(
                BmbpVarsColumn::VarsCodePath,
                RdbcColumn::replace(
                    BmbpVarsColumn::VarsCodePath.get_ident(),
                    old_code_path,
                    new_code_path,
                ),
            );
        update.like_left_(BmbpVarsColumn::VarsCodePath, old_code_path);
        BmbpCurdDao::execute_update::<BmbpVars>(orm, &update).await
    }
}
