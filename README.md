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
