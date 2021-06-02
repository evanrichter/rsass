//! Tests auto-converted from "sass-spec/spec/directives/forward/member/import/import_to_forward/top_level.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .mock_file("mixin/_midstream.scss", "@forward \"upstream\";\n")
        .mock_file("mixin/_upstream.scss", "@mixin a() {b {c: d}}\n")
        .mock_file("post_facto/_midstream.scss", "@forward \"upstream\";\n")
        .mock_file("post_facto/_other.scss", "@mixin a {b {c: $d}}\n")
        .mock_file("post_facto/_upstream.scss", "$d: e;\n")
        .mock_file(
            "variable_assignment/_midstream.scss",
            "@forward \"upstream\";\n",
        )
        .mock_file(
            "variable_assignment/_upstream.scss",
            "$a: old value;\n\n@function get-a() {@return $a}\n",
        )
        .mock_file("variable_use/_midstream.scss", "@forward \"upstream\";\n")
        .mock_file("variable_use/_upstream.scss", "$c: d;\n")
}

#[test]
#[ignore] // unexepected error
fn mixin() {
    let runner = runner().with_cwd("mixin");
    assert_eq!(
        runner.ok("@import \"midstream\";\n\
             \n@include a;\n"),
        "b {\
         \n  c: d;\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn post_facto() {
    let runner = runner().with_cwd("post_facto");
    assert_eq!(
        runner.ok("@import \"other\";\
             \n@import \"midstream\";\n\
             \n@include a;\n"),
        "b {\
         \n  c: e;\
         \n}\n"
    );
}
#[test]
#[ignore] // wrong result
fn variable_assignment() {
    let runner = runner().with_cwd("variable_assignment");
    assert_eq!(
        runner.ok("@import \"midstream\";\n\
             \n$a: new value;\n\
             \nb {c: get-a()}\n"),
        "b {\
         \n  c: new value;\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn variable_use() {
    let runner = runner().with_cwd("variable_use");
    assert_eq!(
        runner.ok("@import \"midstream\";\n\
             \na {b: $c}\n"),
        "a {\
         \n  b: d;\
         \n}\n"
    );
}