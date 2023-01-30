use utility_macros::insert_before_path_tail;

#[test]
fn insert_before_path_tail_macro_ident() {
    fn set_prop() -> bool {
        true
    }
    let func = insert_before_path_tail!(set_, prop);
    assert_eq!(func(), set_prop());
}

#[test]
fn insert_before_path_tail_macro_path() {
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
fn insert_before_path_tail_macro_leading_turbofish_type() {
    struct Struct;
    impl Struct {
        pub fn set_prop() -> bool {
            true
        }
    }
    let func = insert_before_path_tail!(set_, <Struct>::prop);
    assert_eq!(func(), <Struct>::set_prop());
}
