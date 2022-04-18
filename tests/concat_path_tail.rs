use utility_macros::{insert_after_path_tail, insert_before_path_tail};

#[test]
fn insert_before_path_tail_macro() {
    mod module {
        pub fn set_prop() -> bool {
            true
        }
    }
    let func = insert_before_path_tail!(set_, module::prop);
    assert_eq!(func(), module::set_prop());
}

#[test]
fn insert_before_path_tail_macro_leading_double_colon() {
    let func = insert_before_path_tail!(set_, ::test_crate::Struct::prop);
    assert_eq!(func(), ::test_crate::Struct::set_prop());
}

#[test]
fn insert_after_path_tail_macro() {
    mod module {
        pub fn prop_set() -> bool {
            true
        }
    }
    let func = insert_after_path_tail!(_set, module::prop);
    assert_eq!(func(), module::prop_set());
}
