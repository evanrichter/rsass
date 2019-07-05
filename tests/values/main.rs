//! Tests auto-converted from "sass-spec/spec/values"
//! version d8dc38cd, 2019-07-03 17:55:12 -0700.
//! See <https://github.com/sass/sass-spec> for source material.\n
use rsass::{compile_scss, OutputStyle};

mod colors;

mod identifiers;

// From "sass-spec/spec/values/ids.hrx"
#[test]
#[ignore] // failing
fn ids() {
    assert_eq!(
        rsass(
            "// The CSS nav-up property and its siblings allow ID tokens in their values, so\n// Sass parses any tokens that start with a \"#\" followed by an identifier as an\n// ID if it can\'t be parsed as a color.\na {\n  // These IDs are the wrong number of letters to be a hex color.\n  two-letters: #ab;\n  five-letters: #abcde;\n  seven-letters: #abcdefa;\n  nine-letters: #abcdefabc;\n\n  // These IDs contain letters outside the hexadecimal gamut.\n  three-letters-not-hex: #axc;\n  four-letters-not-hex: #axcd;\n  six-letters-not-hex: #abcxde;\n  eight-letters-not-hex: #abcxdefa;\n}\n"
        )
        .unwrap(),
        "a {\n  two-letters: #ab;\n  five-letters: #abcde;\n  seven-letters: #abcdefa;\n  nine-letters: #abcdefabc;\n  three-letters-not-hex: #axc;\n  four-letters-not-hex: #axcd;\n  six-letters-not-hex: #abcxde;\n  eight-letters-not-hex: #abcxdefa;\n}\n"
    );
}

mod lists;

mod maps;

mod numbers;

fn rsass(input: &str) -> Result<String, String> {
    compile_scss(input.as_bytes(), OutputStyle::Expanded)
        .map_err(|e| format!("rsass failed: {}", e))
        .and_then(|s| {
            String::from_utf8(s)
                .map(|s| s.replace("\n\n", "\n"))
                .map_err(|e| format!("{:?}", e))
        })
}
