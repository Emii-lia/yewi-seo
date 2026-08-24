use proc_macro::TokenStream;
use quote::quote;
use syn::{Attribute, ItemFn};
use crate::seo::types::entries::icon::IconEntry;
use crate::seo::types::SeoArgs;

pub mod types;
pub mod dioxus;
pub mod leptos;

pub fn seo_impl(args: SeoArgs, item: ItemFn) -> syn::Result<TokenStream> {
  let ItemFn { attrs, vis, sig, block, modifiers: _modifiers } = item;
  let fn_name = &sig.ident;

  if !attrs.iter().any(|attr: &Attribute|
    attr.path().is_ident("component")
    || attr.path().is_ident("function_component")
  ) {
    return Err(syn::Error::new(
      proc_macro2::Span::call_site()
      ,"#[seo(...)] must be placed above #[component(...)], e.g.:\n\
       #[seo(...)]\n\
       #[component(Foo)]\n\
       fn foo() -> Html { ... }",
    ));
  }

  let seo_meta = match args.meta {
    None => quote! {},
    Some(meta) => meta.build_seo_meta_tokens(),
  };
  let open_graph = match args.open_graph {
    None => quote! {},
    Some(og) => og.build_open_graph_tokens(),
  };
  let twitter = match args.twitter {
    None => quote! {},
    Some(tw) => tw.build_twitter_tokens()
  };

  let link = match args.link {
    None => quote! {},
    Some(link) => link.build_link_tokens()
  };
  let icon = match args.icon {
    None => quote! {},
    Some(icons) => IconEntry::build_icons_tokens(icons)
  };

  let expanded = quote! {
    #(#attrs)*
    #vis #sig {
      ::yew::use_effect_with((), |_| {
        #seo_meta;
        #open_graph;
        #twitter;
        #link;
        #icon;
        || ()
      });
      #block
    }
  };

  let _ = fn_name;
  Ok(TokenStream::from(expanded))
}
