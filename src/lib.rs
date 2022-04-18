mod path;

use path::Path;
use proc_macro::{Ident, Spacing, TokenStream, TokenTree};

#[proc_macro]
pub fn insert_before_path_tail(tokens: TokenStream) -> TokenStream {
    let mut tokens = tokens.into_iter();

    let (ident, path) = parse_args(&mut tokens);

    path::modify_tail(path, |last_segment| {
        let mut ident_string = ident.to_string();
        ident_string.push_str(&last_segment.to_string());

        Ident::new(&ident_string, last_segment.span())
    })
}

#[proc_macro]
pub fn insert_after_path_tail(tokens: TokenStream) -> TokenStream {
    let mut tokens = tokens.into_iter();

    let (ident, path) = parse_args(&mut tokens);

    path::modify_tail(path, |last_segment| {
        let mut ident_string = last_segment.to_string();
        ident_string.push_str(&ident.to_string());

        Ident::new(&ident_string, last_segment.span())
    })
}

fn parse_args(tokens: &mut <proc_macro::TokenStream as IntoIterator>::IntoIter) -> (Ident, Path) {
    let ident = parse_first_arg(tokens);
    parse_comma(tokens);
    let path = parse_second_arg(tokens);

    (ident, path)
}

fn parse_comma(tokens: &mut <proc_macro::TokenStream as IntoIterator>::IntoIter) {
    match tokens.next() {
        Some(TokenTree::Punct(punct))
            if punct.spacing() == Spacing::Alone && punct.as_char() == ',' => {}
        _ => panic!("Expected comma"),
    }
}

fn parse_first_arg(tokens: &mut <proc_macro::TokenStream as IntoIterator>::IntoIter) -> Ident {
    while let Some(token_tree) = tokens.next() {
        if let TokenTree::Ident(ident) = token_tree {
            return ident;
        } else {
            panic!("Expected ident");
        }
    }
    panic!("Expected ident");
}

fn parse_second_arg(tokens: &mut <proc_macro::TokenStream as IntoIterator>::IntoIter) -> Path {
    let (path, next_token) = path::parse(tokens);
    if let Some(next_token) = next_token {
        panic!("Unexpected {next_token}");
    }
    path
}
