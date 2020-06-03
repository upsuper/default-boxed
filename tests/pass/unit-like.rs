use default_boxed::DefaultBoxed;

#[derive(DefaultBoxed)]
struct Foo;

fn main() {
    let _ = Foo::default_boxed();
}
