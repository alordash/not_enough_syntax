use proc_macro2::Span;
use syn::*;

pub fn string(span: Span, string: &str) -> Expr {
    Expr::Lit(ExprLit {
        attrs: Vec::new(),
        lit: Lit::Str(LitStr::new(string, span)),
    })
}

pub fn byte_string(span: Span, byte_string: &[u8]) -> Expr {
    Expr::Lit(ExprLit {
        attrs: Vec::new(),
        lit: Lit::ByteStr(LitByteStr::new(byte_string, span)),
    })
}
