extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro_error::{abort_call_site, proc_macro_error};
use quote::quote;

#[proc_macro_error]
#[proc_macro]
pub fn view(input: TokenStream) -> TokenStream {
    let mut tokens = input.into_iter().peekable();

    let mut style_expr = None;

    // optional: parse `styles = ... ,`
    if let Some(proc_macro::TokenTree::Ident(ident)) = tokens.peek() {
        if ident.to_string() == "styles" {
            tokens.next(); // "styles"
            match tokens.next() {
                Some(proc_macro::TokenTree::Punct(p)) if p.as_char() == '=' => (),
                _ => abort_call_site!("Expected `=` after `styles`"),
            }

            let mut style_expr_str = String::new();
            while let Some(tok) = tokens.peek() {
                if let proc_macro::TokenTree::Punct(p) = tok {
                    if p.as_char() == ',' {
                        tokens.next(); // consume comma
                        break;
                    }
                }
                style_expr_str.push_str(&tokens.next().unwrap().to_string());
            }
            let mut style_expr_str = String::new();
            while let Some(tok) = tokens.peek() {
                if let proc_macro::TokenTree::Punct(p) = tok {
                    if p.as_char() == ',' {
                        tokens.next(); // consume comma
                        break;
                    }
                }
                style_expr_str.push_str(&tokens.next().unwrap().to_string());
            }

            let style_expr_tokens: proc_macro2::TokenStream = style_expr_str.parse().unwrap();
            style_expr = Some(style_expr_tokens);
        }
    }

    let remaining_str = tokens.map(|t| t.to_string()).collect::<String>();
    let remaining: proc_macro2::TokenStream = remaining_str.parse().unwrap();

    let expanded = if let Some(style_expr) = style_expr {
        quote! {
            {
                let styles = #style_expr;
                let $crate::StyleInfo { class_name, style_string } = $crate::get_style_info(styles);
                ::leptos::view! {
                    class={class_name.clone()},
                    <style>{style_string.clone()}</style>
                    #remaining
                }
            }
        }
    } else {
        quote! {
            ::leptos::view! {
                #remaining
            }
        }
    };

    expanded.into()
}
