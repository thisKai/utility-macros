use itertools::{Itertools, Position};
use proc_macro::{Ident, Punct, Spacing, TokenStream, TokenTree};
use std::iter;

pub(crate) fn modify_tail(mut path: Path, modifier: impl Fn(Ident) -> Ident) -> TokenStream {
    let last_segment = path.segments.pop().unwrap();
    let new_last_segment = modifier(last_segment);
    path.segments.push(new_last_segment);

    path.tokens().collect()
}

pub(crate) fn parse(
    tokens: &mut <proc_macro::TokenStream as IntoIterator>::IntoIter,
) -> (Path, Option<TokenTree>) {
    let mut segments = Vec::new();

    let (leading, mut look_for_segment) = {
        match tokens.next().expect("Expected path, unexpected end") {
            TokenTree::Ident(ident) => {
                segments.push(ident);
                (Leading::Ident, false)
            }
            TokenTree::Punct(punct) => match (punct.as_char(), punct.spacing()) {
                (':', Spacing::Joint) => {
                    if parse_second_colon(tokens) {
                        (Leading::DoubleColon, true)
                    } else {
                        panic!("Expected ::");
                    }
                }
                ('<', Spacing::Alone) => {
                    let (path, next_token) = parse(tokens);

                    match next_token {
                        Some(TokenTree::Punct(punct)) if punct.as_char() == '>' => {
                            (Leading::Turbofish(Box::new(path)), false)
                        }
                        Some(other_token) => panic!("Unexpected {other_token}"),
                        None => {
                            panic!("Unexpected end")
                        }
                    }
                }
                (punc, _) => panic!("Expected path, unexpected {punc}"),
            },
            other => panic!("Expected path, unexpected {other}"),
        }
    };

    let mut path = Path { leading, segments };

    while let Some(token_tree) = tokens.next() {
        if look_for_segment {
            match token_tree {
                TokenTree::Ident(ident) => {
                    path.segments.push(ident);
                    look_for_segment = false;
                }
                other => panic!("Expected path segment, unexpected {other}"),
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
pub(crate) struct Path {
    leading: Leading,
    segments: Vec<Ident>,
}
impl Path {
    pub(crate) fn tokens(self) -> impl Iterator<Item = TokenTree> {
        self.leading.tokens().chain(
            self.segments
                .into_iter()
                .map(TokenTree::Ident)
                .with_position()
                .flat_map(|position| match position {
                    Position::First(tt) | Position::Middle(tt) => {
                        iter::once(tt).chain(Some(double_colon()).into_iter().flatten())
                    }
                    Position::Only(tt) | Position::Last(tt) => {
                        iter::once(tt).chain(None.into_iter().flatten())
                    }
                }),
        )
    }
}

pub(crate) fn double_colon() -> [TokenTree; 2] {
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

#[derive(Debug)]
enum Leading {
    Ident,
    DoubleColon,
    Turbofish(Box<Path>),
}
impl Leading {
    fn tokens(self) -> Box<dyn Iterator<Item = TokenTree>> {
        match self {
            Self::Ident => Box::new(None.into_iter()),
            Self::DoubleColon => Box::new(double_colon().into_iter()),
            Self::Turbofish(path) => Box::new(
                iter::once(TokenTree::Punct(Punct::new('<', Spacing::Alone)))
                    .chain(path.tokens())
                    .chain(iter::once(TokenTree::Punct(Punct::new(
                        '>',
                        Spacing::Alone,
                    ))))
                    .chain(double_colon()),
            ),
        }
    }
}
