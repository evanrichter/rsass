//! This module provides `handle_body` (and internally `handle_item`),
//! that does most of the work for [`crate::input::Context::transform`].

// https://github.com/rust-lang/rust-clippy/issues/7846
// https://users.rust-lang.org/t/using-an-option-mut-t-in-a-loop-clippy-complains/72481/2
#![allow(clippy::needless_option_as_deref)]

use super::cssbuf::{CssBuf, CssHead};
use crate::css::{BodyItem, Comment, Import, Property, Rule, Selectors};
use crate::error::{Error, Invalid};
use crate::input::{Context, Loader, Parsed, SourceKind};
use crate::sass::{get_global_module, Expose, Item, UseAs};
use crate::value::ValueRange;
use crate::ScopeRef;
use std::io::Write;

pub fn handle_parsed(
    items: Parsed,
    head: &mut CssHead,
    rule: Option<&mut Rule>,
    buf: &mut CssBuf,
    scope: ScopeRef,
    file_context: &mut Context<impl Loader>,
) -> Result<(), Error> {
    match items {
        Parsed::Scss(items) => {
            handle_body(&items, head, rule, buf, scope, file_context)
        }
        Parsed::Css(items) => {
            for item in items {
                item.write(buf)?;
            }
            Ok(())
        }
    }
}

fn handle_body(
    items: &[Item],
    head: &mut CssHead,
    rule: Option<&mut Rule>,
    buf: &mut CssBuf,
    scope: ScopeRef,
    file_context: &mut Context<impl Loader>,
) -> Result<(), Error> {
    let mut rule = rule;
    for b in items {
        handle_item(
            b,
            head,
            rule.as_deref_mut(),
            buf,
            scope.clone(),
            file_context,
        )?;
    }
    Ok(())
}

fn handle_item(
    item: &Item,
    head: &mut CssHead,
    rule: Option<&mut Rule>,
    buf: &mut CssBuf,
    scope: ScopeRef,
    file_context: &mut Context<impl Loader>,
) -> Result<(), Error> {
    let format = scope.get_format();
    match item {
        Item::Use(ref name, ref as_n, ref with, ref pos) => {
            let name = name.evaluate(scope.clone())?.take_value();
            let module = if let Some(module) = get_global_module(&name) {
                if !with.is_empty() {
                    return Err(Error::BadCall(
                        "Built-in modules can\'t be configured.".into(),
                        pos.clone(),
                        None,
                    ));
                }
                module
            } else if let Some(sourcefile) =
                file_context.find_file(&name, SourceKind::Use(pos.clone()))?
            {
                let module = head.load_module(
                    sourcefile.path(),
                    |head| {
                        let module = ScopeRef::new_global(format);
                        for (name, value, default) in with {
                            let default = if *default {
                                scope.get_or_none(name)
                            } else {
                                None
                            };
                            let value = default.ok_or(()).or_else(|()| {
                                value.do_evaluate(scope.clone(), true)
                            })?;
                            if module.get_or_none(name).is_none() {
                                module.define(name.clone(), value)?;
                            } else {
                                return Err(Error::error(
                                    "The same variable may only be configured once.",
                                ));
                            }
                        }
                        handle_parsed(
                            sourcefile.parse()?,
                            head,
                            None,
                            buf,
                            module.clone(),
                            file_context,
                        )?;
                        Ok(module)
                    })?;
                file_context.unlock_loading(&sourcefile);
                module
            } else {
                return Err(Error::BadCall(
                    "Can't find stylesheet to import.".into(),
                    pos.clone(),
                    None,
                ));
            };
            scope.do_use(module, &name, as_n, &Expose::All)?;
        }
        Item::Forward(ref name, ref as_n, ref expose, ref with, ref pos) => {
            let name = name.evaluate(scope.clone())?.take_value();
            let module = if let Some(module) = get_global_module(&name) {
                if !with.is_empty() {
                    return Err(Error::BadCall(
                        "Built-in modules can\'t be configured.".into(),
                        pos.clone(),
                        None,
                    ));
                }
                module
            } else if let Some(sourcefile) = file_context
                .find_file(&name, SourceKind::Forward(pos.clone()))?
            {
                let module = head.load_module(
                    sourcefile.path(),
                    |head| {
                        let module = ScopeRef::new_global(format);
                        for (name, value, default) in with {
                            let default = if *default {
                                scope.get_or_none(name)
                            } else {
                                None
                            };
                            let value = default.ok_or(()).or_else(|()| {
                                value.do_evaluate(scope.clone(), true)
                            })?;
                            if module.get_or_none(name).is_none() {
                                module.define(name.clone(), value)?;
                            } else {
                                return Err(Error::error(
                                    "The same variable may only be configured once.",
                                ));
                            }
                        }
                        handle_parsed(
                            sourcefile.parse()?,
                            head,
                            None,
                            buf,
                            module.clone(),
                            file_context,
                        )?;
                        Ok(module)
                    });
                file_context.unlock_loading(&sourcefile);
                module?
            } else {
                return Err(Error::S(format!("Module {} not found", name)));
            };
            scope.forward().do_use(module, &name, as_n, expose)?;
        }
        Item::Import(ref names, ref args, ref pos) => {
            let mut rule = rule;
            'name: for name in names {
                let name = name.evaluate(scope.clone())?;
                if args.is_null() {
                    let x = name.value();
                    if let Some(sourcefile) = file_context
                        .find_file(x, SourceKind::Import(pos.clone()))?
                    {
                        match sourcefile.parse()? {
                            Parsed::Scss(items) => {
                                let mut thead = CssHead::new();
                                let module = ScopeRef::sub(scope.clone());
                                handle_body(
                                    &items,
                                    &mut thead,
                                    rule.as_deref_mut(),
                                    buf,
                                    module.clone(),
                                    file_context,
                                )?;
                                head.merge_imports(thead);
                                scope.do_use(
                                    module,
                                    "",
                                    &UseAs::Star,
                                    &Expose::All,
                                )?;
                            }
                            Parsed::Css(items) => {
                                for item in items {
                                    item.write(buf)?;
                                }
                            }
                        }
                        file_context.unlock_loading(&sourcefile);
                        continue 'name;
                    }
                    if !(x.starts_with("http://")
                        || x.starts_with("https://")
                        || x.starts_with("//")
                        || x.ends_with(".css")
                        || name.is_css_url())
                    {
                        return Err(Error::BadCall(
                            "Can't find stylesheet to import.".into(),
                            pos.clone(),
                            None,
                        ));
                    }
                }
                let args = args.evaluate(scope.clone())?;
                let import = Import::new(name, args);
                if let Some(ref mut rule) =
                    rule.as_deref_mut().filter(|r| !r.selectors.is_root())
                {
                    rule.push(import.into());
                } else if buf.is_root_level() {
                    head.add_import(import);
                } else {
                    import.write(buf)?;
                }
            }
        }
        Item::AtRoot(ref selectors, ref body) => {
            let selectors = selectors
                .eval(scope.clone())?
                .with_backref(scope.get_selectors().one());
            let mut rule = Rule::new(selectors.clone());
            let mut sub = CssBuf::new_as(buf);
            handle_body(
                body,
                head,
                Some(&mut rule),
                &mut sub,
                ScopeRef::sub_selectors(scope, selectors),
                file_context,
            )?;
            rule.write(buf)?;
            buf.join(sub);
        }
        Item::AtRule {
            name,
            args,
            body,
            pos: _,
        } => {
            buf.do_separate();
            buf.do_indent_no_nl();
            let name = name.evaluate(scope.clone())?;
            write!(buf, "@{}", name.value())?;
            let args = args.evaluate(scope.clone())?;
            if !args.is_null() {
                write!(buf, " {}", args.format(format))?;
            }
            if let Some(ref body) = *body {
                buf.start_block();
                let selectors = scope.get_selectors().clone();
                let has_selectors = !selectors.is_root();
                let mut rule = Rule::new(selectors);
                let mut sub = CssBuf::new_as(buf);
                handle_body(
                    body,
                    head,
                    Some(&mut rule),
                    &mut sub,
                    ScopeRef::sub(scope),
                    file_context,
                )?;
                if has_selectors {
                    rule.write(buf)?;
                } else {
                    for item in &rule.body {
                        item.write(buf)?;
                    }
                };
                buf.join(sub);
                buf.end_block();
            } else {
                buf.add_one(";\n", ";");
            }
        }

        Item::VariableDeclaration {
            ref name,
            ref val,
            default,
            global,
            ref pos,
        } => {
            let val = val.do_evaluate(scope.clone(), true)?;
            scope
                .set_variable(name.clone(), val, *default, *global)
                .map_err(|e| e.at(pos.clone()))?;
        }
        Item::FunctionDeclaration(ref name, ref body) => {
            if name == "calc"
                || name == "element"
                || name == "expression"
                || name == "url"
            {
                // Ok, this is cheating for the test suite ...
                let p = body.decl.clone().opt_back("@function ");
                return Err(Invalid::FunctionName.at(p));
            }
            check_body(&body.body, BodyContext::Function)?;
            scope.define_function(name.into(), body.closure(&scope).into());
        }
        Item::Return(_, ref pos) => {
            return Err(Invalid::AtRule.at(pos.clone()));
        }

        Item::MixinDeclaration(ref name, ref body) => {
            check_body(&body.body, BodyContext::Mixin)?;
            scope.define_mixin(name.into(), body.closure(&scope).into())
        }
        Item::MixinCall(ref name, ref args, ref body, ref pos) => {
            if let Some(mixin) = scope.get_mixin(&name.into()) {
                let mixin = mixin.get(
                    name,
                    scope.clone(),
                    args,
                    pos,
                    file_context,
                )?;
                mixin.define_content(&scope, body);
                handle_parsed(
                    mixin.body,
                    head,
                    rule,
                    buf,
                    mixin.scope,
                    file_context,
                )
                .map_err(|e: Error| match e {
                    Error::Invalid(err, _) => err.at(pos.clone()),
                    e => {
                        let pos = pos.in_call(name);
                        Error::BadCall(e.to_string(), pos, None)
                    }
                })?;
            } else {
                return Err(Error::BadCall(
                    "Undefined mixin.".into(),
                    pos.clone(),
                    None,
                ));
            }
        }
        Item::Content(args, pos) => {
            if let Some(content) = scope.get_content() {
                let mixin = content.get(
                    "@content",
                    scope,
                    args,
                    pos,
                    file_context,
                )?;
                handle_parsed(
                    mixin.body,
                    head,
                    rule,
                    buf,
                    mixin.scope,
                    file_context,
                )?;
            }
        }

        Item::IfStatement(ref cond, ref do_if, ref do_else) => {
            let cond = cond.evaluate(scope.clone())?.is_true();
            let items = if cond { do_if } else { do_else };
            check_body(items, BodyContext::Control)?;
            handle_body(items, head, rule, buf, scope, file_context)?;
        }
        Item::Each(ref names, ref values, ref body) => {
            check_body(body, BodyContext::Control)?;
            let mut rule = rule;
            let pushed = scope.store_local_values(names);
            for value in values.evaluate(scope.clone())?.iter_items() {
                scope.define_multi(names, value)?;
                handle_body(
                    body,
                    head,
                    rule.as_deref_mut(),
                    buf,
                    scope.clone(),
                    file_context,
                )?;
            }
            scope.restore_local_values(pushed);
        }
        Item::For {
            ref name,
            ref from,
            ref to,
            inclusive,
            ref body,
        } => {
            let range = ValueRange::new(
                from.evaluate(scope.clone())?,
                to.evaluate(scope.clone())?,
                *inclusive,
            )?;
            check_body(body, BodyContext::Control)?;
            let mut rule = rule;
            for value in range {
                let scope = ScopeRef::sub(scope.clone());
                scope.define(name.clone(), value)?;
                handle_body(
                    body,
                    head,
                    rule.as_deref_mut(),
                    buf,
                    scope,
                    file_context,
                )?;
            }
        }
        Item::While(ref cond, ref body) => {
            check_body(body, BodyContext::Control)?;
            let mut rule = rule;
            let scope = ScopeRef::sub(scope);
            while cond.evaluate(scope.clone())?.is_true() {
                handle_body(
                    body,
                    head,
                    rule.as_deref_mut(),
                    buf,
                    scope.clone(),
                    file_context,
                )?;
            }
        }

        Item::Debug(ref value) => {
            eprintln!("DEBUG: {}", value.evaluate(scope)?.format(format));
        }
        Item::Warn(ref value) => {
            eprintln!("WARNING: {}", value.evaluate(scope)?.format(format));
        }
        Item::Error(ref value, ref pos) => {
            return Err(Invalid::AtError(
                value.evaluate(scope)?.format(format).to_string(),
            )
            .at(pos.clone()));
        }

        Item::Rule(ref selectors, ref body) => {
            check_body(body, BodyContext::Rule)?;
            if rule.is_none() {
                buf.do_separate();
            }
            let selectors =
                selectors.eval(scope.clone())?.inside(scope.get_selectors());
            let mut rule = Rule::new(selectors.clone());
            let mut sub = CssBuf::new_as(buf);
            handle_body(
                body,
                head,
                Some(&mut rule),
                &mut sub,
                ScopeRef::sub_selectors(scope, selectors),
                file_context,
            )?;
            rule.write(buf)?;
            buf.join(sub);
        }
        Item::Property(ref name, ref value) => {
            if let Some(rule) = rule {
                let v = value.evaluate(scope.clone())?;
                if !v.is_null() {
                    let name = name.evaluate(scope)?;
                    rule.push(Property::new(name.value().into(), v).into());
                }
            } else {
                return Err(Error::S("Global property not allowed".into()));
            }
        }
        Item::CustomProperty(ref name, ref value) => {
            if let Some(rule) = rule {
                let v = value.evaluate(scope.clone())?;
                if !v.is_null() {
                    let name = name.evaluate(scope)?;
                    rule.push(BodyItem::CustomProperty(
                        name.value().into(),
                        v,
                    ));
                }
            } else {
                return Err(Error::S(
                    "Global custom property not allowed".into(),
                ));
            }
        }
        Item::NamespaceRule(ref name, ref value, ref body) => {
            if let Some(rule) = rule {
                check_body(body, BodyContext::NsRule)?;
                let value = value.evaluate(scope.clone())?;
                let name = name.evaluate(scope.clone())?;
                if !value.is_null() {
                    rule.push(
                        Property::new(name.value().to_string(), value).into(),
                    );
                }
                let mut t = Rule::new(Selectors::root());
                let mut sub = CssBuf::new(format);
                handle_body(
                    body,
                    head,
                    Some(&mut t),
                    &mut sub,
                    scope,
                    file_context,
                )?;
                for item in t.body {
                    rule.push(match item {
                        BodyItem::Property(prop) => {
                            prop.prefix(name.value()).into()
                        }
                        c => c,
                    })
                }
                if !sub.is_empty() {
                    return Err(Error::S(
                        "Unexpected content in namespace rule".into(),
                    ));
                }
            } else {
                return Err(Error::S(
                    "Global namespaced property not allowed".into(),
                ));
            }
        }
        Item::Comment(ref c) => {
            if !format.is_compressed() {
                let c = Comment::from(c.evaluate(scope)?.value());
                if let Some(rule) = rule {
                    rule.push(c.into());
                } else {
                    c.write(buf);
                }
            }
        }

        Item::None => (),
    }
    Ok(())
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum BodyContext {
    Mixin,
    Function,
    Control,
    Rule,
    NsRule,
}

fn check_body(body: &[Item], context: BodyContext) -> Result<(), Error> {
    for item in body {
        match item {
            Item::Forward(_, _, _, _, pos) => {
                return Err(Invalid::AtRule.at(pos.clone()));
            }
            Item::Use(_, _, _, pos) => {
                return Err(Invalid::AtRule.at(pos.clone()));
            }
            Item::MixinDeclaration(.., ref decl) => {
                let pos = decl.decl.clone().opt_back("@mixin ");
                match context {
                    BodyContext::Mixin => {
                        return Err(Invalid::MixinInMixin.at(pos));
                    }
                    BodyContext::Control => {
                        return Err(Invalid::MixinInControl.at(pos));
                    }
                    BodyContext::Rule => (), // This is ok
                    _ => {
                        return Err(Invalid::AtRule.at(pos.opt_trail_ws()));
                    }
                }
            }
            Item::FunctionDeclaration(_, ref body) => {
                let pos = body.decl.clone().opt_back("@function ");
                match context {
                    BodyContext::Mixin => {
                        return Err(Invalid::FunctionInMixin.at(pos));
                    }
                    BodyContext::Control => {
                        return Err(Invalid::FunctionInControl.at(pos));
                    }
                    BodyContext::Rule => (), // This is ok
                    _ => {
                        return Err(Invalid::AtRule.at(pos.opt_trail_ws()));
                    }
                }
            }
            Item::Return(_, ref pos) if context != BodyContext::Function => {
                return Err(Invalid::AtRule.at(pos.clone()));
            }
            Item::AtRule {
                name,
                args: _,
                body: _,
                pos,
            } if context != BodyContext::Rule => {
                if !name
                    .single_raw()
                    .map(|name| CSS_AT_RULES.contains(&name))
                    .unwrap_or(false)
                {
                    return Err(Invalid::AtRule.at(pos.clone()));
                }
            }
            _ => (),
        }
    }
    Ok(())
}

const CSS_AT_RULES: [&str; 16] = [
    "charset",
    "color-profile",
    "counter-style",
    "document",
    "font-face",
    "font-feature-values",
    "import",
    "keyframes",
    "layer",
    "media",
    "namespace",
    "page",
    "property",
    "scroll-timeline",
    "supports",
    "viewport",
];
