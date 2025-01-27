//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1404.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1404")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok(".test {\r\
             \n    color: #aaabbb--1-2-a;\r\
             \n    color: type-of(#aaabbb--1-2-a);\r\
             \n    color: type-of(#aaabbb--1-2);\r\
             \n}"),
        ".test {\
         \n  color: #aaabbb--1-2-a;\
         \n  color: string;\
         \n  color: string;\
         \n}\n"
    );
}
