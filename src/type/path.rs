use crate::path;
use proc_macro2::Span;
use syn::*;

pub fn new<const N: usize>(span: Span, path_parts: [&str; N]) -> TypePath {
    let result = TypePath {
        attrs: Vec::new(),
        qself: None,
        path: path::new(span, path_parts),
    };

    return result;
}

pub fn new_global<const N: usize>(span: Span, path_parts: [&str; N]) -> TypePath {
    let result = TypePath {
        attrs: Vec::new(),
        qself: None,
        path: path::new_global(span, path_parts),
    };

    return result;
}
