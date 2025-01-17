#[allow(dead_code)]
#[rustversion::attr(since(1.36), cfg_attr(not(feature = "web_test"), test))]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/derive_props/pass.rs");
    t.compile_fail("tests/derive_props/fail.rs");
}
