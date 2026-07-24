//! This crate provides `seo` attributes macro to add or edit SEO metadata to a Yew component page.
//! ```
//! use yew::{component, html, Html};
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
//!It also provides `apply_meta`, `apply_open_graph`, `apply_twitter_card`, `apply_link`, `apply_icon` macros to apply SEO metadata individually.
//!
//!```
//!use yew::{component, html, Html};
//! use yewi_seo::{apply_icon, apply_link, apply_meta, apply_open_graph, apply_twitter_card};
//!
//! #[component(About)]
//! pub(crate) fn about() -> Html {
//!   apply_icon!(
//!     (rel = "icon", href = "/favicon.ico")
//!   );
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
//!```
//!
//! Please refer to [https://github.com/Emii-lia/yewi-seo](https:://github.com/Emii-lia/yewi-seo) for the setup.

use proc_macro::TokenStream;
use syn::{parse, parse_macro_input, Error, ItemFn};
use seo::types::SeoArgs;
use crate::seo::seo_impl;
use crate::seo::types::entries::icon::IconEntry;
use crate::seo::types::entries::link::LinkEntry;
use crate::seo::types::entries::meta::MetaEntry;
use crate::seo::types::entries::open_graph::OpenGraphEntry;
use crate::seo::types::entries::twitter::TwitterEntry;

mod seo;

#[proc_macro_attribute]
pub fn seo(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as SeoArgs);
    let item_fn = match parse::<ItemFn>(item.clone()) {
        Ok(f) => f,
        Err(_) => {
            return Error::new(
                proc_macro2::Span::call_site(),
                "#[seo(...)] can only be applied to functions, e.g.:\n\
                 #[seo(...)]\n\
                 #[component(Foo)]\n\
                 fn foo() -> Html { ... }",
            ).to_compile_error().into();
        }
    };

    seo_impl(args, item_fn)
      .unwrap_or_else(|err| TokenStream::from(err.to_compile_error()))
      .into()
}

#[proc_macro]
pub fn apply_meta(attr: TokenStream) -> TokenStream {
    let meta = parse_macro_input!(attr as MetaEntry);
    TokenStream::from(meta.build_seo_meta_tokens())
}

#[proc_macro]
pub fn apply_open_graph(attr: TokenStream) -> TokenStream {
    let og = parse_macro_input!(attr as OpenGraphEntry);
    TokenStream::from(og.build_open_graph_tokens())
}

#[proc_macro]
pub fn apply_twitter_card(attr: TokenStream) -> TokenStream {
    let twitter = parse_macro_input!(attr as TwitterEntry);
    TokenStream::from(twitter.build_twitter_tokens())
}

#[proc_macro]
pub fn apply_link(attr: TokenStream) -> TokenStream {
    let link = parse_macro_input!(attr as LinkEntry);
    TokenStream::from(link.build_link_tokens())
}

#[proc_macro]
pub fn apply_icon(attr: TokenStream) -> TokenStream {
    let icon = parse_macro_input!(attr as IconEntry);
    TokenStream::from(IconEntry::build_icon_tokens(icon))
}