use utility_macros::insert_after_path_tail;

#[test]
fn insert_after_path_tail_macro() {
    mod module {
        pub fn name_suffix() -> bool {
            true
        }
    }
    let func = insert_after_path_tail!(_suffix, module::name);
    assert_eq!(func(), module::name_suffix());
}
