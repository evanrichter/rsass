//! Support for `@import`, `@use`, and `@forward`.
use super::strings::{
    name, sass_string, sass_string_dq, sass_string_sq, special_url,
};
use super::util::{ignore_comments, ignore_space, opt_spacelike};
use super::value::space_list;
use super::{media_args, Span};
use crate::sass::{Expose, Item, Name, SassString, UseAs, Value};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{all_consuming, map, opt, value};
use nom::multi::{fold_many0, separated_list0};
use nom::sequence::{delimited, pair, preceded, terminated, tuple};
use nom::IResult;
use nom_locate::position;
use std::collections::BTreeSet;

/// What follows the `@import` tag.
pub fn import2(input: Span) -> IResult<Span, Item> {
    map(
        terminated(
            tuple((
                position,
                separated_list0(
                    comma,
                    alt((
                        sass_string_dq,
                        sass_string_sq,
                        special_url,
                        sass_string,
                    )),
                ),
                opt(media_args),
            )),
            preceded(
                opt(ignore_space),
                alt((tag(";"), all_consuming(tag("")))),
            ),
        ),
        |(position, import, args)| {
            Item::Import(import, args.unwrap_or(Value::Null), position.into())
        },
    )(input)
}

pub fn use2(input: Span) -> IResult<Span, Item> {
    map(
        terminated(
            tuple((
                terminated(any_sass_string, opt_spacelike),
                opt(preceded(
                    terminated(tag("with"), opt_spacelike),
                    with_arg,
                )),
                opt(preceded(terminated(tag("as"), opt_spacelike), as_arg)),
            )),
            alt((tag(";"), all_consuming(tag("")))),
        ),
        |(s, w, n)| {
            Item::Use(s, n.unwrap_or(UseAs::KeepName), w.unwrap_or_default())
        },
    )(input)
}

pub fn forward2(input: Span) -> IResult<Span, Item> {
    let (mut input, path) =
        terminated(any_sass_string, opt_spacelike)(input)?;
    let mut found_as = None;
    let mut expose = Expose::All;
    let mut found_with = None;
    while let Ok((rest, arg)) = terminated(name, opt_spacelike)(input) {
        input = match arg.as_ref() {
            "as" if found_as.is_none() => {
                let (i, a) = as_arg(rest)?;
                found_as = Some(a);
                i
            }
            "hide" if expose == Expose::All => {
                let (i, (funs, vars)) = exposed_names(rest)?;
                expose = Expose::Hide(funs, vars);
                i
            }
            "show" if expose == Expose::All => {
                let (i, (funs, vars)) = exposed_names(rest)?;
                expose = Expose::Show(funs, vars);
                i
            }
            "with" if found_with.is_none() => {
                let (i, w) = with_arg(rest)?;
                found_with = Some(w);
                i
            }
            _ => {
                return Err(nom::Err::Error(nom::error::Error::new(
                    input,
                    nom::error::ErrorKind::MapRes,
                )));
            }
        };
    }
    let (input, _) = alt((tag(";"), all_consuming(tag(""))))(input)?;
    Ok((
        input,
        Item::Forward(
            path,
            found_as.unwrap_or(UseAs::Star),
            expose,
            found_with.unwrap_or_default(),
        ),
    ))
}

fn exposed_names(
    input: Span,
) -> IResult<Span, (BTreeSet<Name>, BTreeSet<Name>)> {
    let mut funs = BTreeSet::new();
    let mut vars = BTreeSet::new();
    let mut one = pair(
        map(opt(tag("$")), |v| v.is_some()),
        map(terminated(name, opt_spacelike), Name::from),
    );
    let (input, (v, n)) = one(input)?;
    if v { &mut vars } else { &mut funs }.insert(n);
    fold_many0(
        preceded(terminated(tag(","), opt_spacelike), one),
        (funs, vars),
        |(mut funs, mut vars), (v, n)| {
            if v { &mut vars } else { &mut funs }.insert(n);
            (funs, vars)
        },
    )(input)
}

fn as_arg(input: Span) -> IResult<Span, UseAs> {
    terminated(
        alt((
            map(pair(name, opt(tag("*"))), |(name, s)| match s {
                None => UseAs::Name(name),
                Some(_) => UseAs::Prefix(name),
            }),
            value(UseAs::Star, tag("*")),
        )),
        opt_spacelike,
    )(input)
}

fn with_arg(input: Span) -> IResult<Span, Vec<(Name, Value, bool)>> {
    delimited(
        terminated(tag("("), opt_spacelike),
        separated_list0(
            comma,
            tuple((
                delimited(
                    tag("$"),
                    map(name, Name::from),
                    delimited(opt_spacelike, tag(":"), opt_spacelike),
                ),
                terminated(space_list, opt_spacelike),
                map(opt(terminated(tag("!default"), opt_spacelike)), |o| {
                    o.is_some()
                }),
            )),
        ),
        delimited(opt(comma), tag(")"), opt_spacelike),
    )(input)
}

fn any_sass_string(input: Span) -> IResult<Span, SassString> {
    alt((sass_string_dq, sass_string_sq, sass_string))(input)
}

fn comma(input: Span) -> IResult<Span, ()> {
    map(terminated(tag(","), ignore_comments), |_| ())(input)
}
