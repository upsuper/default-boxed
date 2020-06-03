use default_boxed::DefaultBoxed;

#[derive(DefaultBoxed)]
struct Foo {
    _t: (),
}

fn main() {
    let _ = Foo::default_boxed();
}
