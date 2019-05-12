extern crate proc_macro;
extern crate quote;

use quote::quote;
use proc_macro::{TokenStream, TokenTree, Span, Ident, Punct, Spacing, Literal, Group, Delimiter};
use std::usize;

fn parse_c_const(value: &str) -> usize {
    if value.starts_with("0x") {
        usize::from_str_radix(value.trim_start_matches("0x"), 16).unwrap()
    } else if value.starts_with("0b") {
        usize::from_str_radix(value.trim_start_matches("0b"), 2).unwrap()
    } else if value.starts_with("0") {
        usize::from_str_radix(value.trim_start_matches("0"), 8).unwrap()
    } else {
        usize::from_str_radix(value, 10).unwrap()
    }
}

#[proc_macro_attribute]
pub fn preprocessor(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let iters: Vec<TokenTree> = item.into_iter().collect();
    let mut enum_name: Option<String> = None;
    let mut gen_tokens: Vec<TokenTree> = Vec::new();
    let mut enum_tokens: Vec<TokenTree> = Vec::new();
    for it in iters {
        match it {
            TokenTree::Ident(ref ident) => {
                if ident.to_string() == "const" || ident.to_string() == "str" {
                    continue;
                }
                if enum_name.is_some() {
                    panic!("more than only name found");
                }
                enum_name = Some(ident.to_string());
            },
            TokenTree::Literal(ref literal) => {
                let mut data = literal.to_string();
                // remove quotes
                data.remove(data.len() - 1);
                data.remove(0);
                for line in data.split("\n") {
                    let tokens: Vec<&str> = line.split(|ch| ch == ' ' || ch == '\t').filter(|s| s.len() > 0).collect();
                    if tokens.len() != 3 {
                        // no supported
                        panic!("unsupported line: {}", line);
                    }
                    if tokens[0] == "#define" {
                        let name = tokens[1];
                        let value = tokens[2];

                        // constants
                        gen_tokens.push(TokenTree::Ident(Ident::new("const", Span::call_site())));
                        gen_tokens.push(TokenTree::Ident(Ident::new(&format!("{}_{}", enum_name.as_ref().expect("no name found"), name), Span::call_site())));
                        gen_tokens.push(TokenTree::Punct(Punct::new(':', Spacing::Alone)));
                        gen_tokens.push(TokenTree::Ident(Ident::new("usize", Span::call_site())));
                        gen_tokens.push(TokenTree::Punct(Punct::new('=', Spacing::Alone)));
                        gen_tokens.push(TokenTree::Literal(Literal::usize_unsuffixed(parse_c_const(value))));
                        gen_tokens.push(TokenTree::Punct(Punct::new(';', Spacing::Alone)));

                        // enum
                        enum_tokens.push(TokenTree::Ident(Ident::new(name, Span::call_site())));
                        enum_tokens.push(TokenTree::Punct(Punct::new('=', Spacing::Alone)));
                        enum_tokens.push(TokenTree::Literal(Literal::usize_unsuffixed(parse_c_const(value))));
                        enum_tokens.push(TokenTree::Punct(Punct::new(',', Spacing::Alone)));
                    }
                }
            },
            _ => {}
        }
    }
    let mut enum_stream = TokenStream::new();
    enum_stream.extend(enum_tokens);
    let group = Group::new(Delimiter::Brace, enum_stream);

    let debug: TokenStream = quote!(#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]).into();
    gen_tokens.extend(debug);
    gen_tokens.push(TokenTree::Ident(Ident::new("enum", Span::call_site())));
    gen_tokens.push(TokenTree::Ident(Ident::new(enum_name.as_ref().expect("no name found"), Span::call_site())));
    gen_tokens.push(TokenTree::Group(group));

    let mut result = TokenStream::new();
    result.extend(gen_tokens);
    result.into()
}