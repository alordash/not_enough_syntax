use crate::self_expr_path;
use syn::*;

pub fn new(base: Expr, field_ident: Ident) -> ExprField {
    let span = field_ident.span();
    let result = ExprField {
        attrs: Vec::new(),
        base: Box::new(base),
        dot_token: Token![.](span),
        member: Member::Named(field_ident),
    };

    return result;
}

pub fn new_self(field_ident: Ident) -> ExprField {
    let span = field_ident.span();
    let result = ExprField {
        attrs: Vec::new(),
        base: Box::new(Expr::Path(self_expr_path(span))),
        dot_token: Token![.](span),
        member: Member::Named(field_ident),
    };

    return result;
}
