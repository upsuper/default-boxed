use default_boxed::DefaultBoxed;

#[derive(DefaultBoxed)]
struct Foo([u32; 4 * 1024 * 1024], [u16; 4 * 1024 * 1024]);

fn main() {
    let foo = Foo::default_boxed();
    assert_eq!(foo.0[1024], 0);
    assert_eq!(foo.1[2048], 0);
}
