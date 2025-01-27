//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1804/variable.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("variable")
}

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        runner().err(
            "$foo: 2px;\
             \n$bar: 5in;\n\
             \nfoo {\
             \n  bar: #{($foo*$bar)};\
             \n}\n"
        ),
        "Error: 10px*in isn\'t a valid CSS value.\
         \n  ,\
         \n5 |   bar: #{($foo*$bar)};\
         \n  |          ^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 5:10  root stylesheet",
    );
}
