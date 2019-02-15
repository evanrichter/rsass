//! Tests auto-converted from "sass-spec/spec/parser/interpolate/17_escapes_literal_lowercase"
#[allow(unused)]
use super::rsass;
#[allow(unused)]
use rsass::set_precision;

// Ignoring "01_inline.hrx", start_version is 3.7.

// Ignoring "02_variable.hrx", start_version is 3.7.

// Ignoring "03_inline_double.hrx", start_version is 3.7.

// Ignoring "04_variable_double.hrx", start_version is 3.7.

// Ignoring "05_variable_quoted_double.hrx", start_version is 3.7.

/// From "sass-spec/spec/parser/interpolate/17_escapes_literal_lowercase/06_escape_interpolation.hrx"
#[test]
fn t06_escape_interpolation() {
    assert_eq!(
        rsass(
            "$input: \\b\\c\\d\\e\\f\\g\\h\\i\\j\\k\\l\\m\\n\\o\\p\\q\\r\\s\\t\\u\\v\\w\\x\\y\\z;\n.result {\n  output: \"[\\#{\\b\\c\\d\\e\\f\\g\\h\\i\\j\\k\\l\\m\\n\\o\\p\\q\\r\\s\\t\\u\\v\\w\\x\\y\\z}]\";\n  output: \"\\#{\\b\\c\\d\\e\\f\\g\\h\\i\\j\\k\\l\\m\\n\\o\\p\\q\\r\\s\\t\\u\\v\\w\\x\\y\\z}\";\n  output: \'\\#{\\b\\c\\d\\e\\f\\g\\h\\i\\j\\k\\l\\m\\n\\o\\p\\q\\r\\s\\t\\u\\v\\w\\x\\y\\z}\';\n  output: \"[\'\\#{\\b\\c\\d\\e\\f\\g\\h\\i\\j\\k\\l\\m\\n\\o\\p\\q\\r\\s\\t\\u\\v\\w\\x\\y\\z}\']\";\n}\n"
        )
        .unwrap(),
        ".result {\n  output: \"[\\#{\\b\\c\\d\\e\\f\\g\\h\\i\\j\\k\\l\\m\\n\\o\\p\\q\\r\\s\\t\\u\\v\\w\\x\\y\\z}]\";\n  output: \"\\#{\\b\\c\\d\\e\\f\\g\\h\\i\\j\\k\\l\\m\\n\\o\\p\\q\\r\\s\\t\\u\\v\\w\\x\\y\\z}\";\n  output: \'\\#{\\b\\c\\d\\e\\f\\g\\h\\i\\j\\k\\l\\m\\n\\o\\p\\q\\r\\s\\t\\u\\v\\w\\x\\y\\z}\';\n  output: \"[\'\\#{\\b\\c\\d\\e\\f\\g\\h\\i\\j\\k\\l\\m\\n\\o\\p\\q\\r\\s\\t\\u\\v\\w\\x\\y\\z}\']\";\n}\n"
    );
}
