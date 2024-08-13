use bmbp_marco_rdbc::{table_rdbc_tree_bean_orm_option};
use serde::Deserialize;
use serde::Serialize;
use bmbp_rdbc_type::{RdbcIdent, RdbcValue};
use bmbp_rdbc_type::RdbcTable;
use bmbp_rdbc_type::RdbcOrmRow;

#[table_rdbc_tree_bean_orm_option()]
pub struct BmbpDict{
    dict_value:Option<String>,
    dict_alias:Option<String>,
}



#[cfg(test)]
mod bean_test {
    #[test]
    fn test_bean_dict() {
        println!("test_bean");
    }
}

