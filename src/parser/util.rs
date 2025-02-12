use super::{PResult, Span};
use crate::sass::{SassString, StringPart};
use nom::branch::alt;
use nom::bytes::complete::{is_not, tag};
use nom::character::complete::multispace1;
use nom::combinator::{all_consuming, map, map_res, not, opt, peek};
use nom::multi::{fold_many0, fold_many1, many0};
use nom::sequence::{preceded, terminated};
use std::str::from_utf8;

pub fn semi_or_end(input: Span) -> PResult<()> {
    terminated(
        opt_spacelike,
        alt((tag(";"), all_consuming(tag("")), peek(tag("}")))),
    )(input)
}

pub fn spacelike(input: Span) -> PResult<()> {
    fold_many1(alt((ignore_space, ignore_lcomment)), || (), |(), ()| ())(
        input,
    )
}

pub fn spacelike2(input: Span) -> PResult<()> {
    terminated(spacelike, ignore_comments)(input)
}

pub fn opt_spacelike(input: Span) -> PResult<()> {
    fold_many0(alt((ignore_space, ignore_lcomment)), || (), |(), ()| ())(
        input,
    )
}

pub fn ignore_comments(input: Span) -> PResult<()> {
    fold_many0(
        alt((ignore_space, ignore_lcomment, map(comment, |_| ()))),
        || (),
        |(), ()| (),
    )(input)
}

pub fn comment(input: Span) -> PResult<SassString> {
    preceded(tag("/*"), comment2)(input)
}

pub fn comment2(input: Span) -> PResult<SassString> {
    use super::strings::string_part_interpolation;
    use crate::value::Quotes;
    map(
        terminated(
            many0(alt((
                map(
                    map_res(is_not("*#\r\n\u{c}"), |s: Span| {
                        from_utf8(s.fragment())
                    }),
                    StringPart::from,
                ),
                map(
                    alt((tag("\r\n"), tag("\n"), tag("\r"), tag("\u{c}"))),
                    |_| "\n".into(),
                ),
                map(terminated(tag("*"), peek(not(tag("/")))), |_| {
                    StringPart::from("*")
                }),
                string_part_interpolation,
                map(
                    map_res(tag("#"), |s: Span| from_utf8(s.fragment())),
                    StringPart::from,
                ),
            ))),
            tag("*/"),
        ),
        |p| SassString::new(p, Quotes::None),
    )(input)
}

pub fn ignore_space(input: Span) -> PResult<()> {
    map(multispace1, |_| ())(input)
}

fn ignore_lcomment(input: Span) -> PResult<()> {
    map(terminated(tag("//"), opt(is_not("\n"))), |_| ())(input)
}

#[cfg(test)]
mod test {
    use super::comment;

    #[test]
    fn comment_simple() {
        assert_eq!(check_parse!(comment, b"/* hello */"), " hello ".into());
    }

    #[test]
    fn comment_with_stars() {
        assert_eq!(
            check_parse!(comment, b"/**** hello ****/"),
            "*** hello ***".into()
        )
    }

    #[test]
    fn comment_with_stars2() {
        assert_eq!(
            check_parse!(comment, b"/* / * / * / * hello * \\ * \\ * \\ */"),
            " / * / * / * hello * \\ * \\ * \\ ".into()
        )
    }
}
