use utility_macros::insert_after_path_tail;

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
