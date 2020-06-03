use default_boxed::DefaultBoxed;

#[derive(DefaultBoxed)]
struct Foo {
    a: u32,
    b: [[u32; 10 * 1024]; 1024],
}

fn main() {
    let foo = Foo::default_boxed();
    assert_eq!(foo.a, 0);
    assert_eq!(foo.b[512][9 * 1024], 0);
}
