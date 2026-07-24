# Yewi-seo

## About

SEO metadata for [Yew](https://yew.rs) function components that sets your `<title>`, meta tags, Open Graph, Twitter Card, canonical links, and favicons declaratively with a single attribute macro.

Part of the [`yewi`](https://yewi.fiaro.app) ecosystem.

## Installation

```toml
yewi-seo = "^0.1"
```

## Quickstart

```rust
use yew::prelude::*;
use yewi_seo::seo;

#[seo(
  meta(
    title = "Pricing – MyApp",
    description = "Simple, transparent pricing for teams of any size.",
  ),
  open_graph(
    title = "Pricing – MyApp",
    image = "https://myapp.com/og/pricing.png",
    url = "https://myapp.com/pricing",
  ),
  twitter(
    card = "summary_large_image",
    site = "@myapp",
  ),
  link(
    canonical = "https://myapp.com/pricing",
  ),
  icon(
    ( rel = "icon", href = "https://myapp.com/favicon.ico" ),
    ( rel = "apple-touch-icon", href = "https://myapp.com/apple-touch-icon.png" ),
  )
)]
#[component(PricingPage)]
fn pricing_page() -> Html {
  html! {
      <div>{ "Pricing content" }</div>
    }
}

```

`#[seo(...)]` must always be placed **above** `#[component(...)]`. On mount, it injects a `use_effect_with` hook that writes the requested tags into `document.head`.

## What it sets

| Section | Covers                                                                                                                                                                        |
|---|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `meta(...)` | `<title>`, `description`, `application` name, `author`, `generator`, `keywords`, `referrer`, `robots`, `theme-color`, `viewport`, `abstract`, `category`, `classification`    |
| `open_graph(...)` | `og:title`, `og:description`, `og:image` (+ dimensions/alt/type), `og:url`, `og:type`, `og:site_name`, `og:locale` (+ alternates), `og:audio`, `og:video`, and related fields |
| `twitter(...)` | Twitter Card tags — `card`, `site`, `creator`, `title`, `description`, `image` (+ alt/dimensions/type)                                                                        |
| `link(...)` | `<link>` tags : `author`, `manifest`, `canonical`                                                                                                                             |
| `icon(...)` | One or more `<link rel="icon">`: style entries (`href`, `sizes`, `rel`, `color`)                                                                                              |

Every field is optional — include only the sections and keys you need.

## Multiple icons

`icon` accepts a comma-separated list of entries:

```rust
#[seo(
    icon(href = "/favicon-32.png", sizes = "32x32", rel = "icon"),
    icon(href = "/favicon-16.png", sizes = "16x16", rel = "icon"),
    icon(href = "/apple-touch-icon.png", sizes = "180x180", rel = "apple-touch-icon"),
)]
#[component(App)]
fn app() -> Html { html! {} }
```

## Using macros

You can also use the `apply_*` macro to apply SEO metadata individually:

```rust
use yewi_seo::apply_meta;

apply_meta!(
  title = "About - Yewi",
  description = "About page of Yewi",
  keywords = "yew, yewi, about, page",
);
```

Available directly: `apply_meta`, `apply_open_graph`, `apply_twitter_card`, `apply_link`, `apply_icon`.

## Using the runtime functions directly

If you need SEO tags applied outside of the macro (e.g. conditionally, or from non-component code), the underlying functions are public too:

```rust
use yewi_seo::{apply_seo_meta, SeoMetaProps};
 
apply_seo_meta(SeoMetaProps {
    title: Some("Custom title".to_string()),
    ..Default::default()
});
```

Available directly: `apply_seo_meta`, `apply_seo_open_graph`, `apply_seo_twitter_card`, `apply_seo_link`, `apply_icons`, and their matching `*Props` structs.

> Note: `yewi-seo` updates the document head on mount, so it is recommended to call these functions, or macros only once per page. The same applies to `seo` macro attribute.

## Why an attribute macro?

Yew doesn't ship head/meta management out of the box, and reaching for `web_sys::window().document()` boilerplate in every page component gets repetitive fast. `#[seo(...)]` keeps SEO concerns declarative and colocated with the component that owns them, similar in spirit to Next.js's `<Head>` or Nuxt's `useHead`.

## Requirements

- Compiles for `wasm32-unknown-unknown` (browser targets). Tag application is a no-op / not currently supported for SSR contexts.
- Requires `yew` `^0.23` in your own `Cargo.toml`.

## License

MIT

## Contributing

Issues and PRs welcome at [https://github.com/Emii-lia/yewi-seo](https://github.com/Emii-lia/yewi-seo)