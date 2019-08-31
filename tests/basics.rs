use default_boxed::DefaultBoxed;

#[test]
fn test_basic() {
    #[derive(DefaultBoxed)]
    struct Foo {
        a: u32,
        b: [u32; 10 * 1024 * 1024],
    }

    let foo = Foo::default_boxed();
    assert_eq!(foo.a, 0);
    assert_eq!(foo.b[9 * 1024 * 1024], 0);
}

#[test]
fn test_packed_struct() {
    struct A(u8);
    impl Default for A {
        fn default() -> A {
            A(1)
        }
    }

    struct B(u32);
    impl Default for B {
        fn default() -> B {
            B(2)
        }
    }

    struct C(u16);
    impl Default for C {
        fn default() -> C {
            C(3)
        }
    }

    #[derive(DefaultBoxed)]
    struct Foo {
        a: A,
        b: [B; 2],
        c: C,
    }
    // Assert that Rust does pack this struct.
    assert_eq!(std::mem::size_of::<Foo>(), 12);
    let foo = Foo::default_boxed();
    assert_eq!(foo.a.0, 1);
    assert_eq!(foo.b[0].0, 2);
    assert_eq!(foo.b[1].0, 2);
    assert_eq!(foo.c.0, 3);
}