use default_boxed::DefaultBoxed;

#[derive(DefaultBoxed)]
struct Foo {
    b: ([u32; 1024 * 1024], [u32; 1024 * 1024]),
}

fn main() {
    let foo = Foo::default_boxed();
    assert_eq!(foo.b.0[128 * 1024], 0);
    assert_eq!(foo.b.1[256 * 1024], 0);
}
