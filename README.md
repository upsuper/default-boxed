# default-boxed

[![Build Status](https://dev.azure.com/upsuper/upsuper/_apis/build/status/default-boxed?branchName=master)](https://dev.azure.com/upsuper/upsuper/_build/latest?definitionId=1&branchName=master)
[![Crates.io](https://img.shields.io/crates/v/default-boxed.svg)](https://crates.io/crates/default-boxed)

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

<!-- Please keep the code below in sync with tests/readme.rs -->

```rust
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
```
