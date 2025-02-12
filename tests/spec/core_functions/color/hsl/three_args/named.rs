//! Tests auto-converted from "sass-spec/spec/core_functions/color/hsl/three_args/named.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("named")
}

#[test]
fn test() {
    assert_eq!(
        runner()
            .ok("a {b: hsl($hue: 0, $saturation: 100%, $lightness: 50%)}\n"),
        "a {\
         \n  b: hsl(0deg, 100%, 50%);\
         \n}\n"
    );
}
