use utility_macros::insert_before_path_tail;

#[test]
fn insert_before_path_tail_macro_ident() {
    fn prefix_name() -> bool {
        true
    }
    let func = insert_before_path_tail!(prefix_, name);
    assert_eq!(func(), prefix_name());
}

#[test]
fn insert_before_path_tail_macro_path() {
    mod module {
        pub fn prefix_name() -> bool {
            true
        }
    }
    let func = insert_before_path_tail!(prefix_, module::name);
    assert_eq!(func(), module::prefix_name());
}

#[test]
fn insert_before_path_tail_macro_leading_double_colon() {
    let func = insert_before_path_tail!(prefix_, ::test_crate::Struct::name);
    assert_eq!(func(), ::test_crate::Struct::prefix_name());
}

#[test]
fn insert_before_path_tail_macro_leading_turbofish_type() {
    struct Struct;
    impl Struct {
        pub fn prefix_name() -> bool {
            true
        }
    }
    let func = insert_before_path_tail!(prefix_, <Struct>::name);
    assert_eq!(func(), <Struct>::prefix_name());
}
