use crate::*;
use proc_macro2::Span;
use syn::*;

pub fn pub_super(span: Span) -> Visibility {
    let result = Visibility::Restricted(VisRestricted {
        pub_token: Token![pub](span),
        paren_token: token::Paren(span),
        in_token: None,
        path: Box::new(path::new(span, ["super"])),
    });
    return result;
}
