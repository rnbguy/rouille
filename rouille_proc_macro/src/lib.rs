mod traducteur;

use proc_macro::{Group, Ident, TokenStream, TokenTree};

trait Rouille {
    fn traduire(ident_str: &str) -> Option<&str>;

    fn replace_ident(ident: Ident) -> Option<TokenTree> {
        let ident_str = ident.to_string();

        let new_str = Self::traduire(&ident_str)?;

        let new_ident = Ident::new(new_str, ident.span());
        Some(TokenTree::Ident(new_ident))
    }

    fn replace_tree(tok: TokenTree, out: &mut Vec<TokenTree>) {
        match tok {
            TokenTree::Group(group) => {
                let mut group_elem = Vec::new();
                Self::replace_stream(group.stream(), &mut group_elem);
                let mut new_stream = TokenStream::new();
                new_stream.extend(group_elem);
                out.push(TokenTree::Group(Group::new(group.delimiter(), new_stream)));
            }
            TokenTree::Ident(ident) => {
                if let Some(ident) = Self::replace_ident(ident) {
                    out.push(ident);
                }
            }
            TokenTree::Punct(..) | TokenTree::Literal(..) => {
                out.push(tok);
            }
        }
    }

    fn replace_stream(ts: TokenStream, out: &mut Vec<TokenTree>) {
        for tok in ts {
            Self::replace_tree(tok, out)
        }
    }

    fn rouille(item: TokenStream) -> TokenStream {
        let mut returned = Vec::new();
        Self::replace_stream(item, &mut returned);
        let mut out = TokenStream::new();
        out.extend(returned);
        out
    }
}

macro_rules! oxydation {
    ($name:ident, $func:expr) => {
        #[proc_macro]
        pub fn $name(item: TokenStream) -> TokenStream {
            $func(item)
        }
    };
}

#[cfg(feature = "francais")]
oxydation!(rouille, traducteur::francais::Francais::rouille);
