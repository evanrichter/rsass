//! Tests auto-converted from "sass-spec/spec/parser/interpolate/32_comma_list"
#[allow(unused)]
use super::rsass;
#[allow(unused)]
use rsass::set_precision;

/// From "sass-spec/spec/parser/interpolate/32_comma_list/01_inline.hrx"
#[test]
fn t01_inline() {
    assert_eq!(
        rsass(
            ".result {\n  output: \"[\"\',foo,   \'\"]\";\n  output: #{\"[\"\',foo,   \'\"]\"};\n  output: \"[#{\"[\"\',foo,   \'\"]\"}]\";\n  output: \"#{\"[\"\',foo,   \'\"]\"}\";\n  output: \'#{\"[\"\',foo,   \'\"]\"}\';\n  output: \"[\'#{\"[\"\',foo,   \'\"]\"}\']\";\n}\n"
        )
        .unwrap(),
        ".result {\n  output: \"[\" \",foo,   \" \"]\";\n  output: [ ,foo,    ];\n  output: \"[[ ,foo,    ]]\";\n  output: \"[ ,foo,    ]\";\n  output: \"[ ,foo,    ]\";\n  output: \"[\'[ ,foo,    ]\']\";\n}\n"
    );
}

/// From "sass-spec/spec/parser/interpolate/32_comma_list/02_variable.hrx"
#[test]
fn t02_variable() {
    assert_eq!(
        rsass(
            "$input: \"[\"\',foo,   \'\"]\";\n.result {\n  output: $input;\n  output: #{$input};\n  output: \"[#{$input}]\";\n  output: \"#{$input}\";\n  output: \'#{$input}\';\n  output: \"[\'#{$input}\']\";\n}\n"
        )
        .unwrap(),
        ".result {\n  output: \"[\" \",foo,   \" \"]\";\n  output: [ ,foo,    ];\n  output: \"[[ ,foo,    ]]\";\n  output: \"[ ,foo,    ]\";\n  output: \"[ ,foo,    ]\";\n  output: \"[\'[ ,foo,    ]\']\";\n}\n"
    );
}

/// From "sass-spec/spec/parser/interpolate/32_comma_list/03_inline_double.hrx"
#[test]
fn t03_inline_double() {
    assert_eq!(
        rsass(
            ".result {\n  output: #{#{\"[\"\',foo,   \'\"]\"}};\n  output: #{\"[#{\"[\"\',foo,   \'\"]\"}]\"};\n  output: #{\"#{\"[\"\',foo,   \'\"]\"}\"};\n  output: #{\'#{\"[\"\',foo,   \'\"]\"}\'};\n  output: #{\"[\'#{\"[\"\',foo,   \'\"]\"}\']\"};\n}\n"
        )
        .unwrap(),
        ".result {\n  output: [ ,foo,    ];\n  output: [[ ,foo,    ]];\n  output: [ ,foo,    ];\n  output: [ ,foo,    ];\n  output: [\'[ ,foo,    ]\'];\n}\n"
    );
}

/// From "sass-spec/spec/parser/interpolate/32_comma_list/04_variable_double.hrx"
#[test]
fn t04_variable_double() {
    assert_eq!(
        rsass(
            "$input: \"[\"\',foo,   \'\"]\";\n.result {\n  output: #{#{$input}};\n  output: #{\"[#{$input}]\"};\n  output: #{\"#{$input}\"};\n  output: #{\'#{$input}\'};\n  output: #{\"[\'#{$input}\']\"};\n}\n"
        )
        .unwrap(),
        ".result {\n  output: [ ,foo,    ];\n  output: [[ ,foo,    ]];\n  output: [ ,foo,    ];\n  output: [ ,foo,    ];\n  output: [\'[ ,foo,    ]\'];\n}\n"
    );
}

/// From "sass-spec/spec/parser/interpolate/32_comma_list/05_variable_quoted_double.hrx"
#[test]
fn t05_variable_quoted_double() {
    assert_eq!(
        rsass(
            "$input: \"[\"\',foo,   \'\"]\";\n.result {\n  dquoted: \"#{#{$input}}\";\n  dquoted: \"#{\"[#{$input}]\"}\";\n  dquoted: \"#{\"#{$input}\"}\";\n  dquoted: \"#{\'#{$input}\'}\";\n  dquoted: \"#{\"[\'#{$input}\']\"}\";\n  squoted: \'#{#{$input}}\';\n  squoted: \'#{\"[#{$input}]\"}\';\n  squoted: \'#{\"#{$input}\"}\';\n  squoted: \'#{\'#{$input}\'}\';\n  squoted: \'#{\"[\'#{$input}\']\"}\';\n}\n"
        )
        .unwrap(),
        ".result {\n  dquoted: \"[ ,foo,    ]\";\n  dquoted: \"[[ ,foo,    ]]\";\n  dquoted: \"[ ,foo,    ]\";\n  dquoted: \"[ ,foo,    ]\";\n  dquoted: \"[\'[ ,foo,    ]\']\";\n  squoted: \"[ ,foo,    ]\";\n  squoted: \"[[ ,foo,    ]]\";\n  squoted: \"[ ,foo,    ]\";\n  squoted: \"[ ,foo,    ]\";\n  squoted: \"[\'[ ,foo,    ]\']\";\n}\n"
    );
}
