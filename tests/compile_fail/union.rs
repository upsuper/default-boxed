use default_boxed::DefaultBoxed;

#[derive(DefaultBoxed)]
union Foo {
    _a: usize,
}

fn main() {}
