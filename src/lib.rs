use proc_macro::{Ident, Punct, Spacing, TokenStream, TokenTree};
use std::iter;

#[proc_macro]
pub fn insert_before_path_tail(tokens: TokenStream) -> TokenStream {
    let mut tokens = tokens.into_iter();

    let (ident, path) = parse_args(&mut tokens);

    modify_tail(path, |last_segment| {
        let mut ident_string = ident.to_string();
        ident_string.push_str(&last_segment.to_string());

        Ident::new(&ident_string, last_segment.span())
    })
}

#[proc_macro]
pub fn insert_after_path_tail(tokens: TokenStream) -> TokenStream {
    let mut tokens = tokens.into_iter();

    let (ident, path) = parse_args(&mut tokens);

    modify_tail(path, |last_segment| {
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

fn modify_tail(mut path: Path, modifier: impl Fn(Ident) -> Ident) -> TokenStream {
    let new_last_segment = {
        let last_segment = path.segments.pop().unwrap();

        modifier(last_segment)
    };
    path.leading_double_colon
        .then(|| double_colon())
        .into_iter()
        .flatten()
        .chain(
            path.segments
                .into_iter()
                .map(TokenTree::Ident)
                .map(|tt| iter::once(tt).chain(double_colon()))
                .flatten(),
        )
        .chain(iter::once(TokenTree::Ident(new_last_segment)))
        .collect()
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
    let (path, next_token) = parse_path(tokens);
    if let Some(next_token) = next_token {
        panic!("Unexpected {}", next_token);
    }
    path
}

fn parse_path(
    tokens: &mut <proc_macro::TokenStream as IntoIterator>::IntoIter,
) -> (Path, Option<TokenTree>) {
    let mut segments = Vec::new();

    let (leading_double_colon, mut look_for_segment) = {
        match tokens.next().expect("Expected path, unexpected end") {
            TokenTree::Ident(ident) => {
                segments.push(ident);
                (false, false)
            }
            TokenTree::Punct(punct)
                if punct.spacing() == Spacing::Joint && punct.as_char() == ':' =>
            {
                if parse_second_colon(tokens) {
                    (true, true)
                } else {
                    panic!("Expected ::");
                }
            }
            _ => panic!("Expected :: or ident"),
        }
    };

    let mut path = Path {
        leading_double_colon,
        segments,
    };

    while let Some(token_tree) = tokens.next() {
        if look_for_segment {
            match token_tree {
                TokenTree::Ident(ident) => {
                    path.segments.push(ident);
                    look_for_segment = false;
                }
                _ => panic!("Expected path segment"),
            }
        } else {
            match token_tree {
                TokenTree::Punct(punct)
                    if punct.spacing() == Spacing::Joint && punct.as_char() == ':' =>
                {
                    if parse_second_colon(tokens) {
                        look_for_segment = true;
                    } else {
                        panic!("Expected ::");
                    }
                }
                _ => return (path, Some(token_tree)),
            }
        }
    }

    (path, None)
}

#[derive(Debug)]
struct Path {
    leading_double_colon: bool,
    segments: Vec<Ident>,
}

fn double_colon() -> [TokenTree; 2] {
    [
        Punct::new(':', Spacing::Joint),
        Punct::new(':', Spacing::Alone),
    ]
    .map(TokenTree::Punct)
}

fn parse_second_colon(tokens: &mut <proc_macro::TokenStream as IntoIterator>::IntoIter) -> bool {
    match tokens.next() {
        Some(TokenTree::Punct(punct))
            if punct.spacing() == Spacing::Alone && punct.as_char() == ':' =>
        {
            true
        }
        _ => false,
    }
}
