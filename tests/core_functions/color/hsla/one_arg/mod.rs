//! Tests auto-converted from "sass-spec/spec/core_functions/color/hsla/one_arg"
#[allow(unused)]
use super::rsass;
#[allow(unused)]
use rsass::set_precision;

/// From "sass-spec/spec/core_functions/color/hsla/one_arg/alpha.hrx"
#[test]
#[ignore] // failing
fn alpha() {
    assert_eq!(
        rsass(
            "basic {\n  transparent: hsla(180 60% 50% / 0);\n  opaque: hsla(180 60% 50% / 1);\n  partial: hsla(180 60% 50% / 0.5);\n  named: hsla($channels: 180 60% 50% / 0.4);\n\n  // Extra parens shouldn\'t cause the slash to be forced into division.\n  parenthesized: (hsl(180 60% 50% / 0.4));\n}\n\nclamped {\n  saturation: hsla(0 -0.1% 50% / 0.5);\n  lightness: hsla(0 100% 9999% / 0.5);\n  alpha-above: hsla(0 100% 50% / 1.1);\n  alpha-below: hsla(0 100% 50% / -0.1);\n}\n"
        )
        .unwrap(),
        "basic {\n  transparent: rgba(51, 204, 204, 0);\n  opaque: #33cccc;\n  partial: rgba(51, 204, 204, 0.5);\n  named: rgba(51, 204, 204, 0.4);\n  parenthesized: rgba(51, 204, 204, 0.4);\n}\nclamped {\n  saturation: rgba(128, 128, 128, 0.5);\n  lightness: rgba(255, 255, 255, 0.5);\n  alpha-above: red;\n  alpha-below: rgba(255, 0, 0, 0);\n}\n"
    );
}

/// From "sass-spec/spec/core_functions/color/hsla/one_arg/basic.hrx"
#[test]
#[ignore] // failing
fn basic() {
    assert_eq!(
        rsass(
            "basic {\n  red: hsla(0 100% 50%);\n  blue: hsla(240 100% 50%);\n  grayish-yellow: hsla(60 60% 50%);\n}\n\nclamped {\n  saturation-above: hsla(0 500% 50%);\n  saturation-below: hsla(0 -100% 50%);\n  lightness-above: hsla(0 100% 500%);\n  lightness-below: hsla(0 100% -100%);\n}\n\nunits {\n  hue-deg: hsla(0deg 100% 50%);\n  saturation-unitless: hsla(0 50 50%);\n  lightness-unitless: hsla(0 100% 50);\n}\n\nnamed {\n  x: hsla($channels: 0 100% 50%);\n}\n"
        )
        .unwrap(),
        "basic {\n  red: red;\n  blue: blue;\n  grayish-yellow: #cccc33;\n}\nclamped {\n  saturation-above: red;\n  saturation-below: gray;\n  lightness-above: white;\n  lightness-below: black;\n}\nunits {\n  hue-deg: red;\n  saturation-unitless: #bf4040;\n  lightness-unitless: red;\n}\nnamed {\n  x: red;\n}\n"
    );
}

/// From "sass-spec/spec/core_functions/color/hsla/one_arg/special_functions.hrx"
#[test]
#[ignore] // failing
fn special_functions() {
    assert_eq!(
        rsass(
            "no-alpha {\n  calc-1: hsla(calc(1) 2% 3%);\n  calc-2: hsla(1 calc(2%) 3%);\n  calc-3: hsla(1 2% calc(3%));\n\n  var-1: hsla(var(--foo) 2% 3%);\n  var-2: hsla(1 var(--foo) 3%);\n  var-3: hsla(1 2% var(--foo));\n\n  env-1: hsla(env(--foo) 2% 3%);\n  env-2: hsla(1 env(--foo) 3%);\n  env-3: hsla(1 2% env(--foo));\n\n  min-1: hsla(min(1) 2% 3%);\n  min-2: hsla(1 min(2%) 3%);\n  min-3: hsla(1 2% min(3%));\n\n  max-1: hsla(max(1) 2% 3%);\n  max-2: hsla(1 max(2%) 3%);\n  max-3: hsla(1 2% max(3%));\n\n  // var() is substituted before parsing, so it may contain multiple arguments.\n  multi-argument-var-1-of-2: hsla(var(--foo) 50%);\n  multi-argument-var-2-of-2: hsla(0 var(--foo));\n  multi-argument-var-1-of-1: hsla(var(--foo));\n}\n\nalpha {\n  calc-1: hsla(calc(1) 2% 3% / 0.4);\n  calc-2: hsla(1 calc(2%) 3% / 0.4);\n  calc-3: hsla(1 2% calc(3%) / 0.4);\n  calc-4: hsla(1 2% 3% / calc(0.4));\n\n  var-1: hsla(var(--foo) 2% 3% / 0.4);\n  var-2: hsla(1 var(--foo) 3% / 0.4);\n  var-3: hsla(1 2% var(--foo) / 0.4);\n  var-4: hsla(1 2% 3% / var(--foo));\n\n  env-1: hsla(env(--foo) 2% 3% / 0.4);\n  env-2: hsla(1 env(--foo) 3% / 0.4);\n  env-3: hsla(1 2% env(--foo) / 0.4);\n  env-4: hsla(1 2% 3% / env(--foo));\n\n  min-1: hsla(min(1) 2% 3% / 0.4);\n  min-2: hsla(1 min(2%) 3% / 0.4);\n  min-3: hsla(1 2% min(3%) / 0.4);\n  min-4: hsla(1 2% 3% / min(0.4));\n\n  max-1: hsla(max(1) 2% 3% / 0.4);\n  max-2: hsla(1 max(2%) 3% / 0.4);\n  max-3: hsla(1 2% max(3%) / 0.4);\n  max-4: hsla(1 2% 3% / max(0.4));\n\n  // var() is substituted before parsing, so it may contain multiple arguments.\n  multi-argument-var-1-of-2: hsla(var(--foo) 50% / 0.4);\n  multi-argument-var-2-of-2: hsla(0 var(--foo) / 0.4);\n  multi-argument-var-1-of-1: hsla(var(--foo) / 0.4);\n}\n"
        )
        .unwrap(),
        "no-alpha {\n  calc-1: hsla(calc(1), 2%, 3%);\n  calc-2: hsla(1, calc(2%), 3%);\n  calc-3: hsla(1, 2%, calc(3%));\n  var-1: hsla(var(--foo), 2%, 3%);\n  var-2: hsla(1, var(--foo), 3%);\n  var-3: hsla(1, 2%, var(--foo));\n  env-1: hsla(env(--foo), 2%, 3%);\n  env-2: hsla(1, env(--foo), 3%);\n  env-3: hsla(1, 2%, env(--foo));\n  min-1: hsla(min(1), 2%, 3%);\n  min-2: hsla(1, min(2%), 3%);\n  min-3: hsla(1, 2%, min(3%));\n  max-1: hsla(max(1), 2%, 3%);\n  max-2: hsla(1, max(2%), 3%);\n  max-3: hsla(1, 2%, max(3%));\n  multi-argument-var-1-of-2: hsla(var(--foo) 50%);\n  multi-argument-var-2-of-2: hsla(0 var(--foo));\n  multi-argument-var-1-of-1: hsla(var(--foo));\n}\nalpha {\n  calc-1: hsla(calc(1), 2%, 3%, 0.4);\n  calc-2: hsla(1, calc(2%), 3%, 0.4);\n  calc-3: hsla(1 2% calc(3%)/0.4);\n  calc-4: hsla(1 2% 3%/calc(0.4));\n  var-1: hsla(var(--foo), 2%, 3%, 0.4);\n  var-2: hsla(1, var(--foo), 3%, 0.4);\n  var-3: hsla(1 2% var(--foo)/0.4);\n  var-4: hsla(1 2% 3%/var(--foo));\n  env-1: hsla(env(--foo), 2%, 3%, 0.4);\n  env-2: hsla(1, env(--foo), 3%, 0.4);\n  env-3: hsla(1 2% env(--foo)/0.4);\n  env-4: hsla(1 2% 3%/env(--foo));\n  min-1: hsla(min(1), 2%, 3%, 0.4);\n  min-2: hsla(1, min(2%), 3%, 0.4);\n  min-3: hsla(1 2% min(3%)/0.4);\n  min-4: hsla(1 2% 3%/min(0.4));\n  max-1: hsla(max(1), 2%, 3%, 0.4);\n  max-2: hsla(1, max(2%), 3%, 0.4);\n  max-3: hsla(1 2% max(3%)/0.4);\n  max-4: hsla(1 2% 3%/max(0.4));\n  multi-argument-var-1-of-2: hsla(var(--foo) 50%/0.4);\n  multi-argument-var-2-of-2: hsla(0 var(--foo)/0.4);\n  multi-argument-var-1-of-1: hsla(var(--foo)/0.4);\n}\n"
    );
}
