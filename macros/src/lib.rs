/*
    I don't know what I'm doing, but this works

    FIXME: This needs a lot of revision

    References:
    https://belkadan.com/blog/2020/08/Objective-Rust/#recursive-rust-macros
    https://belkadan.com/source/rust-inline-objc/blob/refs/heads/main:/macros/src/lib.rs
    https://doc.rust-lang.org/proc_macro/
    https://docs.rs/proc-macro2/1.0.19/proc_macro2/index.html
    https://docs.rs/quote/1.0.7/quote/index.html
    https://docs.rs/syn/1.0.39/syn/index.html
    https://github.com/dtolnay/syn/blob/master/examples/lazy-static/lazy-static/src/lib.rs
*/

use proc_macro::TokenStream;
use proc_macro2::TokenTree;

use quote::{quote, ToTokens};

use syn::parse::{Parse, ParseStream, Result};
use syn::{parse_macro_input, DeriveInput, Error, Ident, Token, braced, Type, Expr};

/////////////////////////////////////////////////////////////////////////////
// Stateful Macro
/////////////////////////////////////////////////////////////////////////////

struct State {
    pub fields: Vec<(Ident, Type, Expr)>,
    pub map: std::collections::HashMap<String, (Ident, Type)>,
    // body: Vec<Expr>,
    pub view: Expr,
}

impl ToTokens for State {
    // This only turns `self.fields` into tokens, ignoring view
    fn to_tokens(&self, tokens: &mut syn::export::TokenStream2) {
        let mut state_macro_body = syn::export::TokenStream2::new();
        for (field_name, field_type, init_expr) in &self.fields {
            let current = quote! {
                #field_name: #field_type = #init_expr,
            };
            current.to_tokens(&mut state_macro_body);
        }
        
        // let view = &self.view;
        let t = quote! {
            State! {
                #state_macro_body
            };
        };

        t.to_tokens(tokens);
    }
}

impl Parse for State {
    // 1. Extract fields of `@State` item
    // 2. Extract the view expression
    fn parse(input: ParseStream) -> Result<Self> {
        // @
        input.parse::<Token![@]>()?;
        let name: Ident = input.parse()?;
        // @State
        if name != "State" {
            return Err(Error::new(name.span(), format!("Expected `@State`, but got `@{}`", name)));
        }

        // @State { contents... }
        let state_content;
        braced!(state_content in input);

        let mut state_fields = Vec::new();

        // Prevent duplicate identifiers
        let mut field_map = std::collections::HashMap::new();

        // @State { field1: type1 = expr1, ... }
        while !state_content.is_empty() {
            // field
            let field_name: Ident = state_content.parse()?;
            
            if field_map.contains_key(&field_name.to_string()) {
                return Err(Error::new(field_name.span(), format!("Duplicate identifier: `{}`", field_name.to_string())));
            }
            
            // field: type = expr
            state_content.parse::<Token![:]>()?;
            let field_type: Type = state_content.parse()?;
            state_content.parse::<Token![=]>()?;
            let field_init: Expr = state_content.parse()?;
            
            field_map.insert(field_name.to_string(), (field_name.clone(), field_type.clone()));
            
            state_fields.push((field_name, field_type, field_init));

            // Require separating comma and allow one trailing comma
            if !state_content.is_empty() {
                state_content.parse::<Token![,]>()?;
            }
        }

        // @State {..},
        input.parse::<Token![,]>()?;

        // @State {..}, Stack! {...},
        // let body = Punctuated::<Expr, Token![,]>::parse_terminated(input);
        let view: Expr = input.parse()?;

        if !input.is_empty() {
            input.parse::<Token![,]>()?;

            if !input.is_empty() {
                return Err(Error::new(input.span(), "Views are represented by nested views. Only one view should be defined (found multiple)."));
            }
        }

        Ok(State {
            fields: state_fields,
            map: field_map,
            view,
        })
    }
}

// modified from https://belkadan.com/source/rust-inline-objc/blob/refs/heads/main:/macros/src/lib.rs
fn apply_recursive_macro(state: &State, tokens: syn::export::TokenStream2, macro_fn: fn(&State, syn::export::TokenStream2) -> syn::export::TokenStream2) -> impl Iterator<Item = TokenTree> + '_ {
    tokens.into_iter().map(move |tree| {
        if let TokenTree::Group(group) = &tree {
            // println!("group: {}", group);
            let new_contents = macro_fn(state, group.stream());
            let mut result_group = proc_macro2::Group::new(group.delimiter(), new_contents);
            result_group.set_span(group.span());
            TokenTree::Group(result_group)
        } else {
            tree
        }
    })
}

// 1. Traverse token stream
// 2. Check for `@state_field`
// 3. Replace this with `*state.get::<field_type>("state_field")`
// FIXME: Dealing with `@`s here gets ugly
fn format_closures(state: &State, input: syn::export::TokenStream2) -> syn::export::TokenStream2 {
    let mut has_marker: bool = false;
    let mut marker_iter = 0;
    let mut iter = 0;
    let mut should_give_back_at_sign = false;

    apply_recursive_macro(state, input, format_closures).map(|tree| {
        iter += 1;
        if iter > marker_iter + 1 {
            has_marker = false;
        }

        if let TokenTree::Punct(punct) = &tree {
            if punct.as_char() == '@' {
                has_marker = true;
                should_give_back_at_sign = true;
                marker_iter = iter;
                let field = quote! {
                };
                return field.into_token_stream();
            }
        }

        if let TokenTree::Ident(ident) = &tree {
            if has_marker && iter == marker_iter + 1 {
                if let Some((ident, ty)) = state.map.get(&ident.to_string()) {
                    let field = quote! {
                        *state.get::<#ty>(stringify!(#ident))
                    };
                    should_give_back_at_sign = false;
                    return field.into_token_stream();
                }
            }
        }

        if should_give_back_at_sign {
            should_give_back_at_sign = false;
            let at = proc_macro2::Punct::new('@', proc_macro2::Spacing::Alone);
            let mut tokens = tree.into_token_stream();
            at.to_tokens(&mut tokens);
            return tokens;
        }

        tree.into_token_stream()
    }).collect()
}

/// Adds state to a view.
/// State is declared using the following syntax:
///
/// ```
/// @State {
///     field_name: type = initial_expression,
///     ...
/// }
/// ```
/// State can then be used in callbacks like so:
/// ```
/// Widget::new("id")
///     .callback_function(|mut state| {
///         @field_name += 12;
///         println!("Accessing state: {}", @field_name);
///     })
/// ```
/// Note that that `state` **must** be named `state` and nothing else
#[allow(non_snake_case)]
#[proc_macro]
pub fn Stateful(input: TokenStream) -> TokenStream {
    let state = parse_macro_input!(input as State);

    let formatted_view = format_closures(&state, state.view.to_token_stream());
    // println!("View: {}", formatted_view);
    
    let state_expansion = quote! {
        {
            let mut state = #state;
            let mut view = #formatted_view;
            view.assign_state(state);
            view
        }
    };

    // println!("Expanded: {}", state_expansion);

    state_expansion.into()
}

/////////////////////////////////////////////////////////////////////////////
// Derive IntoViewelement
/////////////////////////////////////////////////////////////////////////////

struct ElementDeriveArgs {
    pub kind: Ident,
}

impl Parse for ElementDeriveArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            kind: input.parse()?
        })
    }
}

/// Implements `surreal::IntoViewElement`
///
/// Requires the `kind` attribute to specify which type of view element:
/// - Widget
/// - View
/// - Component
///
/// Note that the generic message type (`Msg`) is required.  
/// If your type does not need generics, use `PhantomData<Msg>`.
///
/// Usage:
/// ```
/// use surreal::view_element::*;
/// ...
/// 
/// #[derive(IntoViewElement)]
/// #[kind(Widget)]
/// pub struct MyWidget<Msg> {...}
///
/// impl<Msg> Widget<Msg> for MyWidget<Msg> {...}
/// ```
#[proc_macro_derive(IntoViewElement, attributes(kind))]
pub fn derive_into_view_element(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);
    
    let kind = if let Some(attr) = derive_input.attrs.get(0) {
        if let Result::Ok(args) = attr.parse_args::<ElementDeriveArgs>() {
            args.kind
        } else {
            panic!("Failed to parse attribute `kind`\nUsage: `#[kind(Widget or View or Component)]`");
        }
    } else {
        panic!("Missing attribute `kind`.\nUsage: `#[kind(Widget or View or Component)]`");
    };
    
    let name = derive_input.ident;

    // TODO: Should generics always be the same? Are there any other cases?
    let expanded = quote! {
        impl<M> IntoViewElement<M> for #name <M> where M: EmptyMessage + 'static {
            fn into_element(self) -> ViewElement<M> {
                ViewElement::#kind(Box::new(self))
            }
        }
    };
        
    expanded.into()
}

/////////////////////////////////////////////////////////////////////////////
// Derive EmptyMessage
/////////////////////////////////////////////////////////////////////////////

struct MessageDeriveArgs {
    pub empty: Ident,
}

impl Parse for MessageDeriveArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            empty: input.parse()?
        })
    }
}

#[proc_macro_derive(EmptyMessage, attributes(empty))]
pub fn derive_empty_message(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);

    let empty = if let Some(attr) = derive_input.attrs.get(0) {
        if let Result::Ok(args) = attr.parse_args::<MessageDeriveArgs>() {
            args.empty
        } else {
            panic!("Failed to parse attribute `empty`\nUsage: `#[empty(EmptyVariant)]`");
        }
    } else {
        panic!("Missing attribute `empty`.\nUsage: `#[empty(EmptyVariant)]`");
    };

    let name = derive_input.ident;

    let expanded = quote! {
        impl EmptyMessage for #name {
            fn is_message(&self) -> bool {
                if let Self::#empty = self {
                    false
                } else {
                    true
                }
            }
        }
    };

    expanded.into()
}