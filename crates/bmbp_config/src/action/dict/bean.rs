use bmbp_marco::bean;
use bmbp_marco::table_rdbc_tree_bean_orm_option;
use bmbp_rdbc::RdbcIdent;
use bmbp_rdbc::RdbcOrmRow;
use bmbp_rdbc::RdbcTable;
use bmbp_util::BmbpTree;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;

#[table_rdbc_tree_bean_orm_option(BMBP_CONFIG_DICT, dict)]
pub struct BmbpDict {
    dict_value: Option<String>,
    dict_alias: Option<String>,
}

#[table_rdbc_tree_bean_orm_option(BMBP_CONFIG_VARS, vars)]
pub struct BmbpVars {
    var_value: Option<String>,
}

#[bean]
pub struct BmbpCombo {
    label: Option<String>,
    value: Option<String>,
    children: Vec<BmbpCombo>,
}

pub(crate) type BmbpCombos = HashMap<String, Vec<BmbpCombo>>;
pub(crate) type BmbpDisplay = HashMap<String, String>;

#[bean]
pub struct BatchReqVo {
    ids: Option<Vec<String>>,
}

#[bean]
pub struct BatchComboVo {
    codes: Option<Vec<String>>,
    cascade: Option<String>,
}
