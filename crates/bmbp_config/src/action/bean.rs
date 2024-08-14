use bmbp_marco_rdbc::{table_rdbc_tree_bean_orm_option};
use serde::Deserialize;
use serde::Serialize;
use bmbp_rdbc_type::{RdbcIdent};
use bmbp_rdbc_type::RdbcTable;
use bmbp_rdbc_type::RdbcOrmRow;
#[table_rdbc_tree_bean_orm_option(BMBP_CONFIG_DICT,dict)]
pub struct BmbpDict{
    dict_value:Option<String>,
    dict_alias:Option<String>,
}
#[table_rdbc_tree_bean_orm_option(BMBP_CONFIG_VARS,vars)]
pub struct BmbpVars{
    var_value:Option<String>,
}



#[cfg(test)]
mod bean_test {
    use bmbp_rdbc_type::RdbcTable;
    use bmbp_rdbc_type::RdbcIdent;
    use crate::action::bean::{BmbpDict, BmbpDictColumn};

    #[test]
    fn test_bean_dict() {
        let dict = BmbpDict::get_table().get_ident();
        assert_eq!("BMBP_CONFIG_DICT",dict.as_str());
    }
}

