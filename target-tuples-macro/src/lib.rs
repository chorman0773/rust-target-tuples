use std::{iter::Peekable, str::FromStr};

use proc_macro::*;
use target_tuple_pieces::*;

use crate::helpers::{AsConstructor, Wildcard};

mod helpers;

struct Error {
    span: Span,
    msg: String,
}

fn emit_error(err: Error) -> TokenStream {
    [
        TokenTree::Punct(Punct::new(':', Spacing::Joint)),
        TokenTree::Punct(Punct::new(':', Spacing::Alone)),
        TokenTree::Ident(Ident::new_raw("core", err.span)),
        TokenTree::Punct(Punct::new(':', Spacing::Joint)),
        TokenTree::Punct(Punct::new(':', Spacing::Alone)),
        TokenTree::Ident(Ident::new_raw("compile_error", err.span)),
        TokenTree::Punct(Punct::new('!', Spacing::Alone)),
        TokenTree::Group(Group::new(
            Delimiter::Parenthesis,
            [TokenTree::Literal(Literal::string(&err.msg))]
                .into_iter()
                .collect(),
        )),
    ]
    .into_iter()
    .collect()
}

fn emit_path<'a>(
    dollar_crate: &TokenStream,
    components: impl IntoIterator<Item = &'a str>,
    span: Span,
) -> impl Iterator<Item = TokenTree> {
    let mut ts = dollar_crate.clone();
    for component in components {
        ts.extend([TokenTree::Punct(Punct::new(':', Spacing::Joint))]);
        ts.extend([TokenTree::Punct(Punct::new(':', Spacing::Alone))]);
        ts.extend([TokenTree::Ident(Ident::new_raw(component, span))]);
    }

    ts.into_iter()
}

#[proc_macro]
pub fn __match_targets(ts: TokenStream) -> TokenStream {
    let mut stream = ts.into_iter();

    let dollar_crate = match stream.next().unwrap() {
        TokenTree::Group(g) => g.stream(),
        _ => panic!("Invalid syntax"),
    };

    match impl_match_targets(&dollar_crate, stream) {
        Ok(ts) => ts,
        Err(e) => emit_error(e),
    }
}

fn impl_match_targets(
    dollar_crate: &TokenStream,
    mut iter: impl Iterator<Item = TokenTree>,
) -> Result<TokenStream, Error> {
    let mut expr = Vec::new();

    let match_body = loop {
        match iter.next() {
            Some(TokenTree::Group(g)) if g.delimiter() == Delimiter::Brace => break g.stream(),
            Some(tt) => expr.push(tt),
            None => {
                return Err(Error {
                    span: Span::call_site(),
                    msg: format!("Unexpected EOF"),
                })
            }
        }
    };

    let expr_span = expr.first().unwrap().span();

    let var_span = expr_span.resolved_at(Span::mixed_site());

    let expr = TokenStream::from_iter(expr);

    let mut arms = TokenStream::new();

    let iter = match_body.into_iter();

    let mut iter = iter.peekable();

    while let Some(v) = parse_match_arm(&mut iter, dollar_crate)? {
        arms.extend(v);
    }

    let mut inner = TokenStream::new();

    let mut fields = TokenStream::new();

    fields.extend([
        TokenTree::Ident(Ident::new_raw("__targ_name", var_span)),
        TokenTree::Punct(Punct::new('.', Spacing::Alone)),
        TokenTree::Ident(Ident::new_raw("arch", var_span)),
        TokenTree::Punct(Punct::new(',', Spacing::Alone)),
        TokenTree::Ident(Ident::new_raw("__targ_name", var_span)),
        TokenTree::Punct(Punct::new('.', Spacing::Alone)),
        TokenTree::Ident(Ident::new_raw("guess_vendor", var_span)),
        TokenTree::Group(Group::new(Delimiter::Parenthesis, TokenStream::new())),
        TokenTree::Punct(Punct::new(',', Spacing::Alone)),
        TokenTree::Ident(Ident::new_raw("__targ_name", var_span)),
        TokenTree::Punct(Punct::new('.', Spacing::Alone)),
        TokenTree::Ident(Ident::new_raw("sys", var_span)),
        TokenTree::Punct(Punct::new('.', Spacing::Alone)),
        TokenTree::Ident(Ident::new_raw("os", var_span)),
        TokenTree::Group(Group::new(Delimiter::Parenthesis, TokenStream::new())),
        TokenTree::Punct(Punct::new(',', Spacing::Alone)),
        TokenTree::Ident(Ident::new_raw("__targ_name", var_span)),
        TokenTree::Punct(Punct::new('.', Spacing::Alone)),
        TokenTree::Ident(Ident::new_raw("sys", var_span)),
        TokenTree::Punct(Punct::new('.', Spacing::Alone)),
        TokenTree::Ident(Ident::new_raw("env", var_span)),
        TokenTree::Group(Group::new(Delimiter::Parenthesis, TokenStream::new())),
        TokenTree::Punct(Punct::new(',', Spacing::Alone)),
        TokenTree::Ident(Ident::new_raw("__targ_name", var_span)),
        TokenTree::Punct(Punct::new('.', Spacing::Alone)),
        TokenTree::Ident(Ident::new_raw("sys", var_span)),
        TokenTree::Punct(Punct::new('.', Spacing::Alone)),
        TokenTree::Ident(Ident::new_raw("object_format", var_span)),
        TokenTree::Group(Group::new(Delimiter::Parenthesis, TokenStream::new())),
        TokenTree::Punct(Punct::new(',', Spacing::Alone)),
    ]);

    inner.extend([
        TokenTree::Ident(Ident::new("let", var_span)),
        TokenTree::Ident(Ident::new_raw("__targ_name", var_span)),
        TokenTree::Punct(Punct::new(':', Spacing::Alone)),
        TokenTree::Punct(Punct::new('&', Spacing::Alone)),
    ]);
    inner.extend(emit_path(dollar_crate, ["CanonicalTarget"], var_span));
    inner.extend([
        TokenTree::Punct(Punct::new('=', Spacing::Alone)),
        TokenTree::Punct(Punct::new('&', Spacing::Alone)),
    ]);
    inner.extend(expr);
    inner.extend([TokenTree::Punct(Punct::new(';', Spacing::Alone))]);
    inner.extend([
        TokenTree::Ident(Ident::new("match", var_span)),
        TokenTree::Group(Group::new(Delimiter::Parenthesis, fields)),
        TokenTree::Group(Group::new(Delimiter::Brace, arms.into_iter().collect())),
    ]);

    Ok(inner)
}

#[derive(Clone, Debug)]
enum Frag {
    WildcardRest,
    WildcardPos,
    Ident(String, Span),
}

fn try_into_frag(tt: TokenTree, has_tail: bool) -> Result<Frag, Error> {
    match tt {
        TokenTree::Punct(p) => {
            if p.as_char() == '*' {
                if has_tail {
                    Ok(Frag::WildcardPos)
                } else {
                    Ok(Frag::WildcardRest)
                }
            } else {
                Err(Error {
                    span: p.span(),
                    msg: format!("Expected `*` or identifier, got `{p}`"),
                })
            }
        }
        TokenTree::Ident(id) => {
            let st = id.to_string();
            let span = id.span();

            if let Some(id) = st.strip_prefix("r#") {
                Ok(Frag::Ident(id.to_string(), span))
            } else {
                Ok(Frag::Ident(st, span))
            }
        }
        tt => Err(Error {
            span: tt.span(),
            msg: format!("Expected `*` or identifier, got `{tt}`"),
        }),
    }
}

#[allow(irrefutable_let_patterns)]
fn parse_match_arm(
    iter: &mut Peekable<impl Iterator<Item = TokenTree>>,
    dcrate: &TokenStream,
) -> Result<Option<TokenStream>, Error> {
    let Some(_) = iter.peek() else {
        return Ok(None);
    };

    let mut left = Vec::new();
    let mut right = Vec::new();

    loop {
        let Some(frag) = iter.next() else {
            return Err(Error {
                span: Span::call_site(),
                msg: format!("Unexpected EOF"),
            });
        };

        match iter.next() {
            Some(TokenTree::Punct(p)) => {
                if p.as_char() == '-' {
                    if left.len() == 4 {
                        return Err(Error {
                            span: p.span(),
                            msg: format!("Expected at most 4 components"),
                        });
                    }
                    left.push(try_into_frag(frag, true)?);
                } else if p.as_char() == '=' && p.spacing() == Spacing::Joint {
                    match iter.next() {
                        Some(TokenTree::Punct(p)) if p.as_char() == '>' => {
                            left.push(try_into_frag(frag, false)?);
                            break;
                        }
                        Some(p) => {
                            return Err(Error {
                                span: p.span(),
                                msg: format!("Expected `=>`, got `{p}`"),
                            })
                        }
                        None => {
                            return Err(Error {
                                span: p.span(),
                                msg: format!("Expected `=>`, got unexpected EOF"),
                            })
                        }
                    }
                } else {
                    return Err(Error {
                        span: p.span(),
                        msg: format!("Expected `-`, got `{p}`"),
                    });
                }
            }
            Some(tt) => {
                return Err(Error {
                    span: tt.span(),
                    msg: format!("Expected `-`, got `{tt}`"),
                })
            }
            None => {
                return Err(Error {
                    span: frag.span(),
                    msg: format!("Expected `-` or `=>`, got unexpected EOF"),
                })
            }
        }
    }

    loop {
        let Some(expr_comp) = iter.next() else {
            return Err(Error {
                span: Span::call_site(),
                msg: format!("Unexpected EOF"),
            });
        };

        right.push(expr_comp.clone());

        match iter.peek() {
            Some(TokenTree::Punct(p)) if p.as_char() == ',' => {
                iter.next();
                break;
            }
            Some(_) => match expr_comp {
                TokenTree::Group(g) if g.delimiter() == Delimiter::Brace => break,
                _ => continue,
            },
            None => break,
        }
    }

    let mut left_match = Vec::new();

    match &*left {
        [Frag::WildcardRest] => {
            left_match.push(TokenTree::Ident(Ident::new("_", Span::call_site())))
        }
        [arch, Frag::WildcardRest] => {
            let mut arch = match arch {
                Frag::Ident(i, span) => {
                    let Ok(piece) = Architecture::from_str(i) else {
                        return Err(Error {
                            span: *span,
                            msg: format!("Unknown architecture {i}"),
                        });
                    };

                    let targ = piece.into_ctor(*span, dcrate);
                    targ
                }
                Frag::WildcardPos => Wildcard.into_ctor(Span::call_site(), dcrate),
                _ => unreachable!(),
            };

            pad_with_wildcard(&mut arch, 1);

            left_match.push(TokenTree::Group(Group::new(Delimiter::Parenthesis, arch)));
        }
        [arch, vendor, Frag::WildcardRest] => {
            let mut arch = match arch {
                Frag::Ident(i, span) => {
                    let Ok(piece) = Architecture::from_str(i) else {
                        return Err(Error {
                            span: *span,
                            msg: format!("Unknown architecture {i}"),
                        });
                    };

                    let targ = piece.into_ctor(*span, dcrate);
                    targ
                }
                Frag::WildcardPos => Wildcard.into_ctor(Span::call_site(), dcrate),
                _ => unreachable!(),
            };

            let vendor = match vendor {
                Frag::Ident(i, span) => {
                    let Ok(piece) = Vendor::from_str(i) else {
                        unreachable!()
                    };

                    let targ = piece.into_ctor(*span, dcrate);
                    targ
                }
                Frag::WildcardPos => Wildcard.into_ctor(Span::call_site(), dcrate),
                _ => unreachable!(),
            };
            arch.extend([TokenTree::Punct(Punct::new(',', Spacing::Alone))]);
            arch.extend(vendor);
            pad_with_wildcard(&mut arch, 2);

            left_match.push(TokenTree::Group(Group::new(Delimiter::Parenthesis, arch)));
        }
        [arch, vendor, Frag::Ident(sys, span)] => {
            let mut arch = match arch {
                Frag::Ident(i, span) => {
                    let Ok(piece) = Architecture::from_str(i) else {
                        return Err(Error {
                            span: *span,
                            msg: format!("Unknown architecture {i}"),
                        });
                    };

                    let targ = piece.into_ctor(*span, dcrate);
                    targ
                }
                Frag::WildcardPos => Wildcard.into_ctor(Span::call_site(), dcrate),
                _ => unreachable!(),
            };

            let vendor = match vendor {
                Frag::Ident(i, span) => {
                    let Ok(piece) = Vendor::from_str(i) else {
                        unreachable!()
                    };

                    let targ = piece.into_ctor(*span, dcrate);
                    targ
                }
                Frag::WildcardPos => Wildcard.into_ctor(Span::call_site(), dcrate),
                _ => unreachable!(),
            };
            let sys = {
                let Ok(piece) = System::from_str(sys) else {
                    return Err(Error {
                        span: *span,
                        msg: format!("Unknown system {sys}"),
                    });
                };

                let os = piece.os();
                let env = piece.env();
                let objfmt = piece.object_format();

                let mut ctor = os.into_ctor(*span, dcrate);

                ctor.extend([TokenTree::Punct(Punct::new(',', Spacing::Alone))]);
                ctor.extend(env.into_ctor(*span, dcrate));
                ctor.extend([TokenTree::Punct(Punct::new(',', Spacing::Alone))]);
                ctor.extend(objfmt.into_ctor(*span, dcrate));

                ctor
            };

            arch.extend([TokenTree::Punct(Punct::new(',', Spacing::Alone))]);
            arch.extend(vendor);
            arch.extend([TokenTree::Punct(Punct::new(',', Spacing::Alone))]);
            arch.extend(sys);

            left_match.push(TokenTree::Group(Group::new(Delimiter::Parenthesis, arch)));
        }
        [arch, vendor, os, envobj] => {
            let mut arch = match arch {
                Frag::Ident(i, span) => {
                    let Ok(piece) = Architecture::from_str(i) else {
                        return Err(Error {
                            span: *span,
                            msg: format!("Unknown architecture {i}"),
                        });
                    };

                    let targ = piece.into_ctor(*span, dcrate);
                    targ
                }
                Frag::WildcardPos => Wildcard.into_ctor(Span::call_site(), dcrate),
                _ => unreachable!(),
            };

            let vendor = match vendor {
                Frag::Ident(i, span) => {
                    let Ok(piece) = Vendor::from_str(i) else {
                        unreachable!()
                    };

                    let targ = piece.into_ctor(*span, dcrate);
                    targ
                }
                Frag::WildcardPos => Wildcard.into_ctor(Span::call_site(), dcrate),
                _ => unreachable!(),
            };
            let os = match os {
                Frag::Ident(os, span) => {
                    let Ok(piece) = OS::from_str(os) else {
                        return Err(Error {
                            span: *span,
                            msg: format!("Unknown operating system {os}"),
                        });
                    };

                    Some(piece).into_ctor(*span, dcrate)
                }
                Frag::WildcardPos => Some(Wildcard).into_ctor(Span::call_site(), dcrate),
                _ => unreachable!(),
            };
            let sys = {
                match envobj {
                    Frag::Ident(i, span) => {
                        let Ok(piece) = System::from_str(i) else {
                            return Err(Error {
                                span: *span,
                                msg: format!("Unknown system {i}"),
                            });
                        };

                        let None = piece.os() else {
                            return Err(Error {
                                span: *span,
                                msg: format!("Operating system {i} not expected"),
                            });
                        };
                        let env = piece.env();
                        let objfmt = piece.object_format();

                        let mut ctor = env.into_ctor(*span, dcrate);

                        ctor.extend([TokenTree::Punct(Punct::new(',', Spacing::Alone))]);
                        ctor.extend(objfmt.into_ctor(*span, dcrate));

                        ctor
                    }
                    _ => {
                        let mut some = Some(Wildcard).into_ctor(Span::call_site(), dcrate);
                        some.extend([TokenTree::Punct(Punct::new('|', Spacing::Alone))]);
                        some.extend(None::<Wildcard>.into_ctor(Span::call_site(), dcrate));

                        let mut rest = some.clone();
                        rest.extend([TokenTree::Punct(Punct::new(',', Spacing::Alone))]);
                        rest.extend(some);

                        rest
                    }
                }
            };

            arch.extend([TokenTree::Punct(Punct::new(',', Spacing::Alone))]);
            arch.extend(vendor);
            arch.extend([TokenTree::Punct(Punct::new(',', Spacing::Alone))]);
            arch.extend(os);
            arch.extend([TokenTree::Punct(Punct::new(',', Spacing::Alone))]);
            arch.extend(sys);

            left_match.push(TokenTree::Group(Group::new(Delimiter::Parenthesis, arch)));
        }
        [.., Frag::Ident(_, span)] => {
            return Err(Error {
                span: *span,
                msg: format!(
                    "Target must have at least 3 components if it doesn't end with a wildcard"
                ),
            })
        }
        _ => unreachable!(),
    }

    let mut tt = left_match.into_iter().collect::<TokenStream>();
    tt.extend([
        TokenTree::Punct(Punct::new('=', Spacing::Joint)),
        TokenTree::Punct(Punct::new('>', Spacing::Alone)),
    ]);
    tt.extend(right);

    tt.extend([TokenTree::Punct(Punct::new(',', Spacing::Alone))]);

    Ok(Some(tt))
}

fn pad_with_wildcard(targ: &mut TokenStream, components: usize) {
    for _ in components..5 {
        targ.extend([
            TokenTree::Punct(Punct::new(',', Spacing::Alone)),
            TokenTree::Ident(Ident::new("_", Span::call_site())),
        ])
    }
}
