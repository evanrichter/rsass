//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/056_test_element_unification_with_namespaceless_element_target.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd(
        "056_test_element_unification_with_namespaceless_element_target",
    )
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("%-a a.foo {a: b}\
             \na {@extend .foo} -a {@extend %-a}\n"),
        "-a a {\
         \n  a: b;\
         \n}\n"
    );
}
