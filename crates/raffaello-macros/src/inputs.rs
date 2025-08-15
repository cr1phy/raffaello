use syn::{Ident, Lit, Token, braced, bracketed, parse::Parse, token};

/// `$key = $value`
pub struct NodeArgument {
    key: Ident,
    equal_token: Token![=],
    value: Lit,
}

impl Parse for NodeArgument {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            key: input.parse()?,
            equal_token: input.parse()?,
            value: input.parse()?,
        })
    }
}

pub enum Node {
    Element {
        name: Ident,
        args: Option<Vec<NodeArgument>>,
        children: Vec<Node>,
    },
    Content(Lit),
}

impl Parse for Node {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if let Ok(literal) = input.parse::<Lit>() {
            Ok(Self::Content(literal))
        } else if input.peek(Ident) {
            let name = input.parse()?;

            let args = match input.peek(token::Bracket) {
                true => {
                    let mut temp_args = vec![];
                    let inner_args;
                    bracketed!(inner_args in input);
                    while !inner_args.is_empty() {
                        temp_args.push(inner_args.parse()?);
                    }
                    Some(temp_args)
                }
                false => None,
            };

            let inner;
            braced!(inner in input);
            let mut children = vec![];
            while !inner.is_empty() {
                children.push(inner.parse()?);
            }

            Ok(Self::Element {
                name,
                args,
                children,
            })
        } else {
            Err(input.error("хуесос, где литерал или другой элемент (`<elem> { ... }`)?"))
        }
    }
}

pub struct RootNode {
    children: Vec<Node>,
}

impl Parse for RootNode {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut children = vec![];
        while !input.is_empty() {
            children.push(input.parse::<Node>()?);
        }
        Ok(Self { children })
    }
}

pub struct RunInput {
    pub compname: Ident,
    pub mode: Ident,
}

impl Parse for RunInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let compname = input.parse()?;
        input.parse::<Token![,]>()?;
        let mode = input.parse()?;
        Ok(Self { compname, mode })
    }
}

pub struct StateInput {}

impl Parse for StateInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {})
    }
}
