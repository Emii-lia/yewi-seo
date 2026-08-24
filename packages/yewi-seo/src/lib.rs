#![allow(clippy::needless_doctest_main)]
#![doc(html_logo_url = "https://yewi.fiaro.app/icons/logo.png")]

//! # Yewi-seo - Documentation
//! Yewi-seo is a SEO metadata for [Yew](https://yew.rs) function components
//! that sets your `<title>`, meta tags, Open Graph, Twitter Card, canonical links, and favicons declaratively with a single attribute macro.
//! Part of the [`yewi`](https://yewi.fiaro.app) ecosystem.
//!
//! ## Features
//! - `#[seo(...)]` macro attributes (`meta(...)`, `open_graph(...)`, `twitter(...)`, `link(...)`, `icon(...)`)
//!   placed **above** `#[function_component(...)]`. On mount, it injects a `use_effect_with` hook that writes the requested tags into `document.head`.
//! - `apply_meta!(...)`, `apply_open_graph!(...)`, `apply_twitter_card!(...)`, `apply_link!(...)` and `apply_icon!(...)` macros that expand into calls to the corresponding apply functions.
//!
//! ## Examples
//!
//! - Using `#[seo(...)]`
//! ```rust, no_run
//! use yew::prelude::*;
//! use yewi_seo::seo;
//!
//! #[seo(
//!   meta(
//!     title = "Yewi",
//!     description = "Component-driven UI kit for Yew",
//!     keywords = "yew, yewi, yew-component, tailwind, scss, component-driven, ui, kit",
//!   ),
//!   open_graph(
//!     title = "Yewi",
//!     description = "Component-driven UI kit for Yew",
//!     url = "https://yewi.fiaro.app",
//!     site_name = "Yewi",
//!     locale = "en_US",
//!     image = "https://yewi.fiaro.app/og-image.png",
//!   ),
//!   link(
//!     canonical = "https://yewi.fiaro.app",
//!   ),
//!   twitter(
//!     card = "summary_large_image",
//!     site = "@Emii_lia",
//!     creator = "@Emii_lia",
//!     title = "Yewi",
//!     description = "Component-driven UI kit for Yew",
//!     image = "https://yewi.fiaro.app/og-image.png",
//!   ),
//!   icon(
//!     ( rel = "icon", href = "/favicon.ico" )
//!   )
//! )]
//! #[component(Home)]
//! fn home() -> Html {
//!   html! {
//!     <div class="Home">
//!       {"Hello, Yewi!"}
//!     </div>
//!   }
//! }
//! ```
//! - Using `apply_*!(...)` macros (available with `apply` feature enabled)
//!
//! ```rust,no_run
//!use yew::{component, html, Html};
//! use yewi_seo::{apply_icon, apply_link, apply_meta, apply_open_graph, apply_twitter_card};
//!
//! #[component(About)]
//! pub(crate) fn about() -> Html {
//!   apply_icon!(rel = "icon", href = "/favicon.ico");
//!   apply_twitter_card!(
//!     card = "summary_large_image",
//!     site = "@Emii_lia",
//!     creator = "@Emii_lia",
//!     title = "About - Yewi",
//!     description = "About page of Yewi",
//!     image = "https://yewi.fiaro.app/images/og.png",
//!   );
//!   apply_link!(
//!     canonical = "https://yewi.fiaro.app",
//!   );
//!   apply_meta!(
//!     title = "About - Yewi",
//!     description = "About page of Yewi",
//!     keywords = "yew, yewi, about, page",
//!   );
//!   apply_open_graph!(
//!     title = "About - Yewi",
//!     description = "About page of Yewi",
//!     url = "https://yewi.fiaro.app",
//!     site_name = "Yewi",
//!     locale = "en_US",
//!     image = "https://yewi.fiaro.app/images/og.png",
//!   );
//!
//!   html! {
//!     <div class="About">
//!       <h1>{"About"}</h1>
//!       <p>{"This is the about page."}</p>
//!     </div>
//!   }
//! }
//! ```

#![allow(macro_expanded_macro_exports_accessed_by_absolute_paths)]
extern crate self as yewi_seo;

pub mod utils;
pub mod meta;
pub mod open_graph;
pub mod link;
pub mod traits;
pub mod icon;
pub mod twitter;

#[cfg(feature = "dioxus")]
pub mod dioxus;

#[cfg(feature = "leptos")]
pub mod leptos;

#[cfg(feature = "yew")]
pub mod yew;

pub use yewi_seo_macro::seo;

#[cfg(feature = "apply")]
pub use yewi_seo_macro::apply_meta;

#[cfg(feature = "apply")]
pub use yewi_seo_macro::apply_open_graph;

#[cfg(feature = "apply")]
pub use yewi_seo_macro::apply_twitter_card;

#[cfg(feature = "apply")]
pub use yewi_seo_macro::apply_link;

#[cfg(feature = "apply")]
pub use yewi_seo_macro::apply_icon;


pub use meta::types::SeoMetaProps;
pub use open_graph::types::OpenGraphProps;
pub use twitter::types::TwitterCardProps;
pub use link::types::LinkProps;
pub use icon::types::IconProps;
pub use icon::types::SeoIconProps;
pub use meta::apply_seo_meta;
pub use open_graph::apply_seo_open_graph;
pub use twitter::apply_seo_twitter_card;
pub use link::apply_seo_link;
pub use icon::apply_seo_icons;
