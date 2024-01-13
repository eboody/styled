// pub use styled_macro::view;
use regex::Regex;
use stylist::{Result, Style as Styles};

pub use leptos::*;
use leptos_dom::HydrationCtx;
pub use leptos_meta::Style;

pub use stylist::style;

#[macro_export]
macro_rules! view {
    ($styles:expr, $($tokens:tt)*) => {{

        let style = $styles;

        let $crate::StyleInfo { class_name, style_string } = $crate::get_style_info(style);
        use $crate::Style;
        ::leptos::view! {
            class={class_name.clone()},
            <Style>{style_string.clone()}</Style>
            $($tokens)*
        }
    }};
}

pub fn get_style_info(styles_result: Result<Styles>) -> StyleInfo {
    let hydration_context_id = HydrationCtx::peek_always();

    let style_struct = styles_result.unwrap();

    let class_name = String::from("styled-") + &hydration_context_id.to_string();

    let style_string = style_struct.get_style_str().to_owned();

    style_struct.unregister();

    let re = Regex::new(r"stylist-\w+").unwrap();

    let style_string = re.replace_all(&style_string, &class_name);

    let re = Regex::new(r"(\.styled(-\d+)+) (-?[_a-zA-Z\.#~]+[_a-zA-Z0-9-]*+)").unwrap();

    let regex_to_fix_stylist_bug = Regex::new(r"(\dpx)([-])").unwrap();

    let style_string_with_fixed_pixels = regex_to_fix_stylist_bug
        .replace_all(&style_string, "$1 $2")
        .to_string();

    let new_style_string = re
        .replace_all(&style_string_with_fixed_pixels, "$3$1")
        .to_string();

    StyleInfo {
        class_name,
        style_string: new_style_string,
    }
}

#[derive(Clone)]
pub struct StyleInfo {
    pub class_name: String,
    pub style_string: String,
}
