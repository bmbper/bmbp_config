use bmbp_marco_rdbc::table_rdbc_tree_bean_orm_option;
use bmbp_rdbc_type::RdbcIdent;
use bmbp_rdbc_type::RdbcOrmRow;
use bmbp_rdbc_type::RdbcTable;
use serde::Deserialize;
use serde::Serialize;
#[table_rdbc_tree_bean_orm_option(BMBP_CONFIG_DICT, dict)]
pub struct BmbpDict {
    dict_value: Option<String>,
    dict_alias: Option<String>,
}
#[table_rdbc_tree_bean_orm_option(BMBP_CONFIG_VARS, vars)]
pub struct BmbpVars {
    var_value: Option<String>,
}
