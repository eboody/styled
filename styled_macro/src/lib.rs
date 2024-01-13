extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro2::TokenTree;
use quote::quote;

#[macro_use]
extern crate proc_macro_error;

#[proc_macro]
pub fn view(tokens: TokenStream) -> TokenStream {
    let tokens: proc_macro2::TokenStream = tokens.into();
    let mut tokens = tokens.into_iter();
    tokens.next();
    let comma = tokens.next();

    match comma {
        Some(TokenTree::Punct(punct)) if punct.as_char() == ',' => {
            let first = tokens.next();
            let second = tokens.next();
            let third = tokens.next();
            let fourth = tokens.next();

            let styles_result = match (&first, &second) {
                (Some(TokenTree::Ident(first)), Some(TokenTree::Punct(eq)))
                    if *first == "styles" && eq.as_char() == '=' =>
                {
                    match &fourth {
                        Some(TokenTree::Punct(comma)) if comma.as_char() == ',' => third.clone(),
                        _ => {
                            abort!(
                                punct, "To create scoped styles with the view! macro you must put a comma `,` after the value";
                                help = r#"e.g., view!{cx, styles={my_styles_result}, <div>...</div>}"#
                            )
                        }
                    }
                }
                _ => None,
            };

            let rest_of_tokens = tokens.collect::<proc_macro2::TokenStream>();

            let output = quote! {
                let hydration_context_id = leptos_dom::HydrationCtx::peek();
                let style_struct = #styles_result.unwrap();
                let class_name = format!("styled-{}", hydration_context_id);
                let style_string = style_struct.get_style_str().to_owned();

                style_struct.unregister();

                let mut style_string_with_fixed_pixels = String::new();
                let mut in_pixel_value = false;

                for (index, c) in style_string.chars().enumerate() {
                    if !in_pixel_value && c == '-' {
                        let char_one_over = &style_string.chars().nth(index - 1);
                        let char_two_over = &style_string.chars().nth(index - 2);
                        let char_three_over = &style_string.chars().nth(index - 3);
                        if let (Some(char_one), Some(char_two), Some(char_three)) =
                            (char_one_over, char_two_over, char_three_over)
                        {
                            if *char_one == 'x' && *char_two == 'p' && char_three.is_ascii_digit() {
                                in_pixel_value = true;
                            }
                        }
                    }

                    if in_pixel_value {
                        style_string_with_fixed_pixels.push(' ');
                        in_pixel_value = false;
                    }

                    style_string_with_fixed_pixels.push(c);
                }

                let mut new_style_string = String::new();
                for line in style_string_with_fixed_pixels.lines() {
                    if let Some(class_idx) = line.find(".stylist-") {
                        if let Some(sel_idx) = line[class_idx..].find(|c: char| c.is_ascii_whitespace()) {
                            let class = &line[class_idx..class_idx + sel_idx];
                            let sel = match &line.chars().nth(&line.len() - 1) {
                                Some('{') => &line[class_idx + sel_idx..line.len() - 1],
                                _ => &line[class_idx + sel_idx..],
                            };

                            new_style_string.push('\n');
                            new_style_string.push_str(sel.trim());
                            new_style_string.push('.');
                            new_style_string.push_str(&class_name);
                            new_style_string.push_str(" {");
                        }
                    } else {
                        new_style_string.push('\n');
                        new_style_string.push_str(line);
                    }
                }
                view! {
                    cx,
                    class={class_name.clone()},
                    <Style>{new_style_string.clone()}</Style>
                    #rest_of_tokens
                }
            };

            output.into()
        }
        _ => {
            abort_call_site!(
                "view! macro needs a context and RSX: e.g., view! {{ cx, \
                 <div>...</div> }}"
            )
        }
    }
}
