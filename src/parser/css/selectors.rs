use super::super::util::{ignore_comments, opt_spacelike, spacelike2};
use super::super::{input_to_string, PResult, Span};
use super::strings::{css_string, css_string_any};
use crate::css::{Selector, SelectorPart, Selectors};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::one_of;
use nom::combinator::{into, map, map_res, opt, value};
use nom::multi::{many1, separated_list1};
use nom::sequence::{delimited, pair, terminated, tuple};

pub fn selectors(input: Span) -> PResult<Selectors> {
    map(
        separated_list1(terminated(tag(","), ignore_comments), selector),
        Selectors::new,
    )(input)
}

pub fn selector(input: Span) -> PResult<Selector> {
    let (input, mut s) = many1(selector_part)(input)?;
    if s.last() == Some(&SelectorPart::Descendant) {
        s.pop();
    }
    Ok((input, Selector(s)))
}

pub fn selector_part(input: Span) -> PResult<SelectorPart> {
    let (input, mark) =
        alt((tag("*"), tag("&"), tag("::"), tag(":"), tag("["), tag("")))(
            input,
        )?;
    match *mark.fragment() {
        b"*" => value(SelectorPart::Simple("*".into()), tag(""))(input),
        b"&" => value(SelectorPart::BackRef, tag(""))(input),
        b"::" => map(
            pair(
                into(css_string),
                opt(delimited(tag("("), selectors, tag(")"))),
            ),
            |(name, arg)| SelectorPart::PseudoElement { name, arg },
        )(input),
        b":" => map(
            pair(
                into(css_string),
                opt(delimited(tag("("), selectors, tag(")"))),
            ),
            |(name, arg)| SelectorPart::Pseudo { name, arg },
        )(input),
        b"[" => delimited(
            opt_spacelike,
            alt((
                map(
                    tuple((
                        terminated(css_string, opt_spacelike),
                        terminated(
                            map_res(
                                alt((
                                    tag("*="),
                                    tag("|="),
                                    tag("="),
                                    tag("$="),
                                    tag("~="),
                                    tag("^="),
                                )),
                                input_to_string,
                            ),
                            opt_spacelike,
                        ),
                        terminated(css_string_any, opt_spacelike),
                        opt(terminated(
                            one_of(
                                "ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                                 abcdefghijklmnopqrstuvwxyz",
                            ),
                            opt_spacelike,
                        )),
                    )),
                    |(name, op, val, modifier)| SelectorPart::Attribute {
                        name: name.into(),
                        op,
                        val,
                        modifier,
                    },
                ),
                map(terminated(css_string, opt_spacelike), |name| {
                    SelectorPart::Attribute {
                        name: name.into(),
                        op: "".to_string(),
                        val: "".into(),
                        modifier: None,
                    }
                }),
            )),
            tag("]"),
        )(input),
        b"" => alt((
            map(css_string, SelectorPart::Simple),
            delimited(
                opt_spacelike,
                alt((
                    value(SelectorPart::RelOp(b'>'), tag(">")),
                    value(SelectorPart::RelOp(b'+'), tag("+")),
                    value(SelectorPart::RelOp(b'~'), tag("~")),
                    value(SelectorPart::RelOp(b'\\'), tag("\\")),
                )),
                opt_spacelike,
            ),
            value(SelectorPart::Descendant, spacelike2),
        ))(input),
        _ => unreachable!(),
    }
}
