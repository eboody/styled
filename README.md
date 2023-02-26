# Styled: Easy Styling for Leptos Components

If you're looking for an easy way to apply scoped styles to your [`Leptos`](https://github.com/leptos-rs/leptos) components, `Styled` is the Leptos macro you need. With `Styled`, you can apply high-level selectors like `button` or `div` to specific components, keeping your markup clean and organized.

## Installation

Use `cargo add` in your project root

```bash
cargo add styled stylist
```

## Usage

First create a basic `Leptos` component. This will serve as the foundation for this little guide.

```rust
#[component]
pub fn MyComponent(cx: Scope) -> impl IntoView{
  view! {
    cx,
    <div>"hello"</div>
  }
}
```

Next, import the `style` macro, powered by an awesome crate called [`Stylist`](https://github.com/futursolo/stylist-rs), to create your styles.
Just add this to the top of your file.

```rust
use styled::style;
```

You can then use the `style` macro to create a `Result` containing your styles. Let's modify our component:

```rust
#[component]
pub fn MyComponent(cx: Scope) -> impl IntoView{
  
  let styles = style!(
    div {
      background-color: red;
      color: white;
    }
  );

  view! {
    cx,
    <div>"hello"</div>
  }
}
```

Now, let's apply those styles with our `styled::view!` macro!

```rust
#[component]
pub fn MyComponent(cx: Scope) -> impl IntoView {

    let styles = style!(
      div {
        background-color: red;
        color: white;
      }
    );

    styled::view! {
        cx,
        styles,
        <div>"This text should be red with white text."</div>
    }
}
```

Now we can define another component that also uses the `div` CSS selector but it's styles will only apply to the elements inside of it's enclosing `styled::view!` macro.

```rust
#[component]
pub fn AnotherComponent(cx: Scope) -> impl IntoView {

    // note were using a plain div selector and it wont clash with MyComponent's div style!
    let styles = style!(
      div {
        background-color: blue;
        color: gray;
      }
    );

    styled::view! {
        cx,
        styles,
        <div>"This text should be blue with gray text."</div>
    }
}
```

## Longer Example

```rust
// /src/components/button.rs

use crate::theme::get_theme;
use leptos::*;
use styled::style;

#[derive(PartialEq)]
pub enum Variant {
    PRIMARY,
    SECONDARY,
    ALERT,
    DISABLED,
}

impl Variant {
    pub fn is(&self, variant: &Variant) -> bool {
        self == variant
    }
}

struct ButtonColors {
    text: String,
    background: String,
    border: String,
}

fn get_colors(variant: &Variant) -> ButtonColors {
    let theme = get_theme().unwrap();
    match variant {
        Variant::PRIMARY => ButtonColors {
            text: theme.white(),
            background: theme.black(),
            border: theme.transparent(),
        },
        Variant::SECONDARY => ButtonColors {
            text: theme.black(),
            background: theme.white(),
            border: theme.gray.lightest(),
        },
        Variant::ALERT => ButtonColors {
            text: theme.white(),
            background: theme.red(),
            border: theme.transparent(),
        },
        Variant::DISABLED => ButtonColors {
            text: theme.white(),
            background: theme.red(),
            border: theme.transparent(),
        },
    }
}

#[component]
pub fn Button(cx: Scope, variant: Variant) -> impl IntoView {
    let disabled = variant.is(&Variant::DISABLED);

    let styles = styles(&variant);

    styled::view! {
        cx,
        styles,
        <button disabled=disabled>"Button"</button>
    }
}

fn styles<'a>(variant: &Variant) -> styled::Result<styled::Style> {
    let colors = get_colors(variant);

    style!(
            button {
                color: ${colors.text};
                background-color: ${colors.background};
                border: 1px solid ${colors.border};
                outline: none;
                height: 48px;
                min-width: 154px;
                font-size: 14px;
                font-weight: 700;
                text-align: center;
                box-shadow: rgba(0, 0, 0, 0.05) 0px 1px 2px 0px;
                position: relative;
                box-sizing: border-box;
                vertical-align: middle;
                text-align: center;
                text-overflow: ellipsis;
                text-transform: uppercase;
                overflow: hidden;
                cursor: pointer;
                transition: box-shadow 0.2s;
                margin: 10px;
            }

            & button:active {
                transform: scale(0.99);
            }


            & button::-moz-focus-inner {
                border: none;
            }

            & button::before {
                content: "";
                position: absolute;
                top: 0;
                bottom: 0;
                left: 0;
                right: 0;
                background-color: rgb(255, 255, 255);
                opacity: 0;
                transition: opacity 0.2s;
            }

            & button::after {
                content: "";
                position: absolute;
                left: 50%;
                top: 50%;
                border-radius: 50%;
                padding: 50%;
                background-color: ${colors.text};
                opacity: 0;
                transform: translate(-50%, -50%) scale(1);
                transition: opacity 1s, transform 0.5s;
            }

            & button:hover,
            & button:focus {
                box-shadow: 0 2px 4px -1px rgba(0, 0, 0, 0.2), 0 4px 5px 0 rgba(0, 0, 0, 0.14), 0 1px 10px 0 rgba(0, 0, 0, 0.12);
            }

            & button:hover::before {
                opacity: 0.08;
            }

            & button:hover:focus::before {
                opacity: 0.3;
            }

            & button:active {
                box-shadow: 0 5px 5px -3px rgba(0, 0, 0, 0.2), 0 8px 10px 1px rgba(0, 0, 0, 0.14), 0 3px 14px 2px rgba(0, 0, 0, 0.12);
            }

            & button:active::after {
                opacity: 0.32;
                transform: translate(-50%, -50%) scale(0);
                transition: transform 0s;
            }

            & button:disabled {
                color: rgba(0, 0, 0, 0.28);
                background-color: rgba(0, 0, 0, 0.12);
                box-shadow: none;
                cursor: initial;
            }

            & button:disabled::before {
                opacity: 0;
            }

            & button:disabled::after {
                opacity: 0;
            }

    )
}

```

```rust
// /src/theme/mod.rs
use csscolorparser::Color;

pub fn get_theme() -> Result<Theme, csscolorparser::ParseColorError> {
    let theme = Theme {
        teal: Colors {
            main: Color::from_html("#6FDDDB")?,
            darker: Color::from_html("#2BB4B2")?,
            lighter: Color::from_html("#7EE1DF")?,
            lightest: Color::from_html("#B2EDEC")?,
        },
        pink: Colors {
            main: Color::from_html("#E93EF5")?,
            darker: Color::from_html("#C70BD4")?,
            lighter: Color::from_html("#F5A4FA")?,
            lightest: Color::from_html("#FCE1FD")?,
        },
        green: Colors {
            main: Color::from_html("#54D072")?,
            darker: Color::from_html("#30AF4F")?,
            lighter: Color::from_html("#82DD98")?,
            lightest: Color::from_html("#B4EAC1")?,
        },
        purple: Colors {
            main: Color::from_html("#8C18FB")?,
            darker: Color::from_html("#7204DB")?,
            lighter: Color::from_html("#B162FC")?,
            lightest: Color::from_html("#D0A1FD")?,
        },
        yellow: Colors {
            main: Color::from_html("#E1E862")?,
            darker: Color::from_html("#BAC31D")?,
            lighter: Color::from_html("#EFF3AC")?,
            lightest: Color::from_html("#FAFBE3")?,
        },
        gray: Colors {
            main: Color::from_html("#4a4a4a")?,
            darker: Color::from_html("#3d3d3d")?,
            lighter: Color::from_html("#939393")?,
            lightest: Color::from_html("#c4c4c4")?,
        },
        red: Color::from_html("#FF5854")?,
        black: Color::from_html("#000000")?,
        white: Color::from_html("#FFFFFF")?,
        transparent: Color::from_html("transparent")?,
    };

    Ok(theme)
}

pub struct Theme {
    pub teal: Colors,
    pub pink: Colors,
    pub green: Colors,
    pub purple: Colors,
    pub yellow: Colors,
    pub gray: Colors,
    pub red: Color,
    pub black: Color,
    pub white: Color,
    pub transparent: Color,
}

pub struct Colors {
    pub main: Color,
    pub darker: Color,
    pub lighter: Color,
    pub lightest: Color,
}

impl Colors {
    pub fn main(&self) -> String {
        self.main.to_hex_string()
    }
    pub fn darker(&self) -> String {
        self.darker.to_hex_string()
    }
    pub fn lighter(&self) -> String {
        self.lighter.to_hex_string()
    }
    pub fn lightest(&self) -> String {
        self.lightest.to_hex_string()
    }
}

impl Theme {
    pub fn red(&self) -> String {
        self.red.to_hex_string()
    }
    pub fn black(&self) -> String {
        self.black.to_hex_string()
    }
    pub fn white(&self) -> String {
        self.white.to_hex_string()
    }
    pub fn transparent(&self) -> String {
        self.transparent.to_hex_string()
    }
}


```

```rust
// /src/app.rs

#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    view! { cx,
            <Button variant={button::Variant::PRIMARY}/>
            <Button variant={button::Variant::SECONDARY}/>
            <Button variant={button::Variant::ALERT}/>
    }
}
```
