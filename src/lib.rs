pub extern crate leptos_meta;
use regex::Regex;
pub use stylist::{style, Result, Style};

#[allow(unused)]
#[macro_export]
macro_rules! view {
    ($cx: expr, $styles:expr, $($tokens:tt)*) => {{
        use $crate::leptos_meta::{Style, StyleProps};
        let v = $cx;
        let style = $styles;

        let $crate::StyleInfo { class_name, styles } = $crate::get_style_info(style);

        view! {
            v,
            class={class_name.clone()},
            <Style>{styles.clone()}</Style>
            $($tokens)*
        }
    }};
}

pub fn get_style_info(styles_result: Result<Style>) -> StyleInfo {
    let style_struct = styles_result.unwrap();

    let class_name = style_struct.get_class_name().to_owned();
    let style_string = style_struct.get_style_str().to_owned();

    style_struct.unregister();

    let re = Regex::new(r"(\.stylist-\w+) (\w+)").unwrap();

    let styles = re.replace_all(&style_string, "$2$1").to_string();

    StyleInfo { class_name, styles }
}

#[derive(Clone)]
pub struct StyleInfo {
    pub class_name: String,
    pub styles: String,
}
