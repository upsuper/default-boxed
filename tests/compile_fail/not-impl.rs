use default_boxed::DefaultBoxed;

struct Bar;

#[derive(DefaultBoxed)]
struct Foo {
    a: Bar,
}

fn main() {}
