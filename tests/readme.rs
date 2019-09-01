// Please keep the code below in sync with README.md
// When cfg(doctest) gets stabilized, we can use doc-comment for running test in README.md.

use default_boxed::DefaultBoxed;

#[derive(DefaultBoxed)]
struct Foo {
    a: Bar,
    b: [Bar; 1024 * 1024],
    c: [u32; 1024 * 1024],
}

struct Bar(u16);
impl Default for Bar {
    fn default() -> Bar {
        Bar(29)
    }
}

#[test]
fn test_basic() {
    let foo = Foo::default_boxed();
    assert_eq!(foo.a.0, 29);
}
