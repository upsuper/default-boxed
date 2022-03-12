# default-boxed

[![CI](https://github.com/upsuper/default-boxed/workflows/CI/badge.svg)](https://github.com/upsuper/default-boxed/actions)
[![Crates.io](https://img.shields.io/crates/v/default-boxed.svg)](https://crates.io/crates/default-boxed)

<!-- cargo-sync-readme start -->

Helper trait to create instances of large structs with default value on heap directly
without going through stack.

Similar to the unstable `box` syntax,
it semantically doesn't require creating the whole struct on stack then moving to heap,
and thus unlike [`copyless`][copyless] or [`boxext`][boxext],
it doesn't rely on optimization to eliminate building the struct on stack,
which may still face stack overflow on debug build when creating large struct.

[copyless]: https://crates.io/crates/copyless
[boxext]: https://crates.io/crates/boxext

## Example

```rust
use default_boxed::DefaultBoxed;

const BASE: usize = 1024;

#[derive(DefaultBoxed)]
struct Foo {
    a: Bar,
    b: [Bar; 1024 * BASE],
    c: [u32; 1024 * BASE],
}

struct Bar(u16);
impl Default for Bar {
    fn default() -> Bar {
        Bar(29)
    }
}

let foo = Foo::default_boxed();
assert_eq!(foo.a.0, 29);
assert_eq!(foo.b[128 * BASE].0, 29);
assert_eq!(foo.c[256 * BASE], 0);

let foo_arr = Foo::default_boxed_array::<16>();
assert_eq!(foo_arr[15].a.0, 29);
```

<!-- cargo-sync-readme end -->
