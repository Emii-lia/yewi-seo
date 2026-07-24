#[allow(dead_code)]
#[rustversion::attr(stable(1.95.0), test)]
fn tests() {
  let t = trybuild::TestCases::new();
  t.pass("tests/seo/*_pass.rs");
  t.compile_fail("tests/seo/*_fail.rs");
}