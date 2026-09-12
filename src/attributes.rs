use crate::path;
use proc_macro2::Span;
use quote::ToTokens;
use syn::*;

pub fn inline(span: Span) -> Attribute {
    let result = Attribute {
        pound_token: Token![#](span),
        style: AttrStyle::Outer,
        bracket_token: token::Bracket(span),
        meta: Meta::Path(path::new(span, ["inline"])),
    };
    return result;
}

pub fn doc_hidden(span: Span) -> Attribute {
    let result = Attribute {
        pound_token: Token![#](span),
        style: AttrStyle::Outer,
        bracket_token: token::Bracket(span),
        meta: Meta::List(MetaList {
            path: path::new(span, ["doc"]),
            delimiter: MacroDelimiter::Paren(token::Paren(span)),
            tokens: Ident::new("hidden", span).to_token_stream(),
        }),
    };
    return result;
}

pub fn allow_clippy(span: Span, lint: &str) -> Attribute {
    let result = Attribute {
        pound_token: Token![#](span),
        style: AttrStyle::Outer,
        bracket_token: token::Bracket(span),
        meta: Meta::List(MetaList {
            path: path::new(span, ["allow"]),
            delimiter: MacroDelimiter::Paren(token::Paren(span)),
            tokens: path::new(span, ["clippy", lint]).to_token_stream(),
        }),
    };
    return result;
}

pub fn allow_unused(span: Span) -> Attribute {
    allow(span, "unused")
}

pub fn allow_nonstandard_style(span: Span) -> Attribute {
    allow(span, "nonstandard_style")
}

pub fn allow_unreachable_pub(span: Span) -> Attribute {
    allow(span, "unreachable_pub")
}

pub fn allow_private_interfaces(span: Span) -> Attribute {
    allow(span, "private_interfaces")
}

pub fn allow_private_bounds(span: Span) -> Attribute {
    allow(span, "private_bounds")
}

pub fn allow_refining_impl_trait(span: Span) -> Attribute {
    allow(span, "refining_impl_trait")
}

fn allow(span: Span, allowed: &'static str) -> Attribute {
    let result = Attribute {
        pound_token: Token![#](span),
        style: AttrStyle::Outer,
        bracket_token: token::Bracket(span),
        meta: Meta::List(MetaList {
            path: path::new(span, ["allow"]),
            delimiter: MacroDelimiter::Paren(token::Paren(span)),
            tokens: Ident::new(allowed, span).to_token_stream(),
        }),
    };
    return result;
}
