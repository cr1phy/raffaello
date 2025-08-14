use syn::{braced, parse::Parse, token, Ident, Lit, Token};

pub enum Node {
    Element {
        name: Ident,
        /* args: Option<NodeArguments> */ children: Vec<Node>,
    },
    Content(Lit),
}

impl Parse for Node {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if let Ok(literal) = input.parse::<Lit>() {
            Ok(Self::Content(literal))
        } else if input.peek(Ident) {
            let name = input.parse()?;
            let inner;
            braced!(inner in input);
            let mut children = vec![];
            while !inner.is_empty() {
                children.push(inner.parse()?);
            }
            Ok(Self::Element { name, children })
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
