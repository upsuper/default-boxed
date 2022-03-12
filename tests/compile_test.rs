#[test]
#[cfg_attr(miri, ignore)]
fn compile_test() {
    let t = trybuild::TestCases::new();
    t.pass("tests/pass/*.rs");
    t.compile_fail("tests/compile_fail/*.rs");
}
