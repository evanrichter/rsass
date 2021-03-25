//! Tests auto-converted from "sass-spec/spec/libsass/selector_interpolation_in_string.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "foo[val=\"bar #{\"foo\" + \" bar\"} baz\"] {a: b}\
            \n"
        )
        .unwrap(),
        "foo[val=\"bar foo bar baz\"] {\
        \n  a: b;\
        \n}\
        \n"
    );
}