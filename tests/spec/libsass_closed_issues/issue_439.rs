//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_439.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_439")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("@mixin odd( $selector, $n) {\
             \n  $selector: \"& + \" + $selector + \" + \" + $selector;\
             \n  $placeholder: unique_id();\
             \n  %#{$placeholder} { @content; }\
             \n  #{$selector}:first-child {\
             \n    #{$selector} { @extend %#{$placeholder}; }\
             \n  }\
             \n}\n\
             \nul > {\
             \n  @include odd( li, 5 ) { background: #ccc;  }\
             \n}\n"),
        ""
    );
}
