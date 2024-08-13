#[test]
fn test_bean_dict() {
    let dict = BmbpDict::new();
    assert_eq!(dict.dict_value, None);
    assert_eq!(dict.dict_alias, None);
}