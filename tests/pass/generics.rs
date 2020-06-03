use default_boxed::DefaultBoxed;

#[derive(DefaultBoxed)]
struct Foo<T> {
    a: T,
    b: [T; 100],
}

struct X(u32);
impl Default for X {
    fn default() -> X {
        X(10)
    }
}

fn main() {
    let foo = Foo::<X>::default_boxed();
    assert_eq!(foo.a.0, 10);
    assert_eq!(foo.b[10].0, 10);
}
