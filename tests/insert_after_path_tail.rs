use utility_macros::insert_after_path_tail;

#[test]
fn insert_after_path_tail_macro_ident() {
    fn name_suffix() -> bool {
        true
    }
    let func = insert_after_path_tail!(_suffix, name);
    assert_eq!(func(), name_suffix());
}

#[test]
fn insert_after_path_tail_macro_path() {
    mod module {
        pub fn name_suffix() -> bool {
            true
        }
    }
    let func = insert_after_path_tail!(_suffix, module::name);
    assert_eq!(func(), module::name_suffix());
}

#[test]
fn insert_after_path_tail_macro_leading_double_colon() {
    let func = insert_after_path_tail!(_suffix, ::test_crate::Struct::name);
    assert_eq!(func(), ::test_crate::Struct::name_suffix());
}

#[test]
fn insert_after_path_tail_macro_leading_turbofish_type() {
    struct Struct;
    impl Struct {
        pub fn name_suffix() -> bool {
            true
        }
    }
    let func = insert_after_path_tail!(_suffix, <Struct>::name);
    assert_eq!(func(), <Struct>::name_suffix());
}
