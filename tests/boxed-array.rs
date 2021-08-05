use default_boxed::DefaultBoxed;

#[test]
fn test_boxed_array() {
    struct Foo(u32);

    impl Default for Foo {
        fn default() -> Self {
            Foo(0x12345678)
        }
    }

    let array = Foo::default_boxed_array::<1024>();
    assert!(array.iter().all(|item| item.0 == 0x12345678));
}
