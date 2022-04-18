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
    let mut segments = Vec::new();

    let (leading_double_colon, mut look_for_segment) = {
        match tokens.next().expect("Expected path, unexpected end") {
            TokenTree::Punct(punct)
                if punct.spacing() == Spacing::Joint && punct.as_char() == ':' =>
            {
                match tokens.next() {
                    Some(TokenTree::Punct(punct))
                        if punct.spacing() == Spacing::Alone && punct.as_char() == ':' =>
                    {
                        (true, true)
                    }
                    _ => panic!("Expected :: or ident"),
                }
            }
            TokenTree::Ident(ident) => {
                segments.push(ident);
                (false, false)
            }
            _ => panic!("Expected :: or ident"),
        }
    };
    while let Some(token_tree) = tokens.next() {
        if look_for_segment {
            match token_tree {
                TokenTree::Ident(ident) => {
                    segments.push(ident);
                    look_for_segment = false;
                }
                _ => panic!("Expected ident"),
            }
        } else {
            match token_tree {
                TokenTree::Punct(punct)
                    if punct.spacing() == Spacing::Joint && punct.as_char() == ':' =>
                {
                    match tokens.next() {
                        Some(TokenTree::Punct(punct))
                            if punct.spacing() == Spacing::Alone && punct.as_char() == ':' =>
                        {
                            look_for_segment = true;
                        }
                        _ => panic!("Expected ::"),
                    }
                }
                _ => panic!("Expected ::"),
            }
        }
    }

    Path {
        leading_double_colon,
        segments,
    }
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
