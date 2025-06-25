use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    sync::atomic::{AtomicUsize, Ordering},
};

pub use stylist::{Result, Style, style};

#[macro_export]
macro_rules! view {
    ($styles:expr, $($tokens:tt)*) => {{

        let style = $styles;

        let $crate::StyleInfo { class_name, style_string } = $crate::get_style_info(style);

        ::leptos::view! {
            <style>{style_string.clone()}</style>
            <div class={class_name}>
                $($tokens)*
            </div>
        }
    }};
}

/// Global counter as fallback for unique class generation
static COUNTER: AtomicUsize = AtomicUsize::new(0);

#[derive(Clone)]
pub struct StyleInfo {
    pub class_name: String,
    pub style_string: String,
}

fn generate_callsite_hash(file: &str, line: u32) -> u64 {
    let mut hasher = DefaultHasher::new();
    file.hash(&mut hasher);
    line.hash(&mut hasher);
    COUNTER.fetch_add(1, Ordering::Relaxed).hash(&mut hasher);
    hasher.finish()
}

pub fn get_style_info(styles_result: Result<Style>) -> StyleInfo {
    let style_struct = styles_result.expect("Style creation failed");

    let original_class = style_struct.get_class_name();
    let class_name = format!("styled-{}", generate_callsite_hash(file!(), line!()));

    // replace only the internal class name
    let raw = style_struct.get_style_str();
    let replaced = raw.replace(original_class, &class_name);

    // Optional: remove px- fix if stylist is fixed
    let cleaned = replaced.replace("px-", "px -");

    StyleInfo {
        class_name,
        style_string: cleaned,
    }
}
