use proc_macro::{Ident, Punct, Spacing, TokenStream, TokenTree};
use std::iter;

#[proc_macro]
pub fn insert_before_path_tail(tokens: TokenStream) -> TokenStream {
    let mut tokens = tokens.into_iter();

    let ident = parse_first_arg(&mut tokens);
    parse_comma(&mut tokens);
    let mut path = parse_second_arg(&mut tokens);

    let new_last_segment = {
        let last_segment = path.segments.pop().unwrap();

        let mut ident_string = ident.to_string();
        ident_string.push_str(&last_segment.to_string());

        Ident::new(&ident_string, last_segment.span())
    };

    path.segments
        .into_iter()
        .map(TokenTree::Ident)
        .map(|tt| {
            iter::once(tt).chain(
                [
                    Punct::new(':', Spacing::Joint),
                    Punct::new(':', Spacing::Alone),
                ]
                .map(TokenTree::Punct),
            )
        })
        .flatten()
        .chain(iter::once(TokenTree::Ident(new_last_segment)))
        .collect()
}

#[proc_macro]
pub fn insert_after_path_tail(tokens: TokenStream) -> TokenStream {
    let mut tokens = tokens.into_iter();

    let ident = parse_first_arg(&mut tokens);
    parse_comma(&mut tokens);
    let mut path = parse_second_arg(&mut tokens);

    let new_last_segment = {
        let last_segment = path.segments.pop().unwrap();

        let mut ident_string = last_segment.to_string();
        ident_string.push_str(&ident.to_string());

        Ident::new(&ident_string, last_segment.span())
    };

    path.segments
        .into_iter()
        .map(TokenTree::Ident)
        .map(|tt| {
            iter::once(tt).chain(
                [
                    Punct::new(':', Spacing::Joint),
                    Punct::new(':', Spacing::Alone),
                ]
                .map(TokenTree::Punct),
            )
        })
        .flatten()
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

    let mut look_for_segment = true;
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

    Path { segments }
}
#[derive(Debug)]
struct Path {
    segments: Vec<Ident>,
}
