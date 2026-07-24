# yewi-seo-macros

Procedural macro crate powering [`yewi-seo`](https://crates.io/crates/yewi-seo)'s `#[seo(...)]` attribute macro and `apply_meta`, `apply_link`, `apply_open_graph`, `apply_twitter_card` and `apply_icon` macros.

> **You almost certainly don't want to depend on this crate directly.** Add [`yewi-seo`](https://crates.io/crates/yewi-seo) instead — it re-exports the macros along with the runtime functions the macro's generated code calls into.

## What this crate does

`yewi-seo-macros` defines a single `#[proc_macro_attribute]`, `seo`, which:

1. Parses the DSL inside `#[seo(...)]` (`meta(...)`, `open_graph(...)`, `twitter(...)`, `link(...)`, `icon(...)`) into a typed AST using [`syn`](https://crates.io/crates/syn).
2. Validates that the macro is applied above a plain `fn` (i.e. **before** `#[function_component(...)]` expands it into a struct/impl).
3. Generates a `::yew::use_effect_with(...)` block that calls the matching `apply_*` runtime functions from `yewi-seo`, and splices it into the top of the function body.

It also defines 5 other `#[proc_macro]` macros (`apply_meta`, `apply_open_graph`, `apply_twitter_card`, `apply_link`, `apply_icon`) that expand into calls to the corresponding runtime functions.

## Macro ordering

```rust
// Correct — #[seo] above #[function_component]
#[seo(meta(title = "Home"))]
#[function_component(Home)]
fn home() -> Html { html! {} }
```

```rust
// Incorrect — will fail to parse as a plain fn
#[function_component(Home)]
#[seo(meta(title = "Home"))]
fn home() -> Html { html! {} }
```

If `#[seo(...)]` isn't the outermost attribute, it receives an already-expanded struct/impl instead of a function, and the macro emits a compile error explaining the required order.

## License

MIT