use stylist::{Result, Style as Styles};

pub use leptos::*;
use leptos_dom::HydrationCtx;

pub use stylist::style;

#[macro_export]
macro_rules! view {
    ($cx: expr, $styles:expr, $($tokens:tt)*) => {{

        let cx = $cx;
        let style = $styles;

        let $crate::StyleInfo { class_name, style_string } = $crate::get_style_info(style);

        view! {
            cx,
            class={class_name.clone()},
            <style>{style_string.clone()}</style>
            $($tokens)*
        }.into_view(cx)
    }};
}

pub fn get_style_info(styles_result: Result<Styles>) -> StyleInfo {
    let hydration_context_id = HydrationCtx::peek();
    let style_struct = styles_result.unwrap();
    let class_name = format!("styled-{}", hydration_context_id);

    let mut style_string = style_struct.get_style_str().to_owned();
    style_struct.unregister();

    // Replace "stylist-\w+" with `class_name`
    if let Some(start_index) = style_string.find("stylist-") {
        let end_index = style_string[start_index..]
            .find(|c: char| !c.is_alphanumeric())
            .map_or_else(|| style_string.len(), |end| end + start_index);
        style_string.replace_range(start_index..end_index, &class_name);
    }

    // Fix stylist bug by adding space between "px" and "-"
    style_string = style_string.replace("px-", "px -");

    // Replace ".styled(-\d+)+ (-?[_a-zA-Z\.#~]+[_a-zA-Z0-9-]*+)" with "$3$1"
    let styled_pattern = format!(".{}", class_name);
    if let Some(start_index) = style_string.find(&styled_pattern) {
        let end_index =
            find_next_whitespace(&style_string, start_index).unwrap_or(style_string.len());
        let selector = &style_string[end_index..]
            .split_whitespace()
            .next()
            .unwrap_or("");
        let replacement = format!("{} {}", selector, &styled_pattern);
        style_string.replace_range(start_index..end_index + selector.len(), &replacement);
    }

    StyleInfo {
        class_name,
        style_string,
    }
}

fn find_next_whitespace(s: &str, from_index: usize) -> Option<usize> {
    s[from_index..]
        .chars()
        .position(|c| c.is_whitespace())
        .map(|i| i + from_index)
}

#[derive(Clone)]
pub struct StyleInfo {
    pub class_name: String,
    pub style_string: String,
}
