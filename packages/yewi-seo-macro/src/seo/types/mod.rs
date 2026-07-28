use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{parenthesized, Token};
use crate::seo::types::entries::icon::IconEntry;
use crate::seo::types::entries::link::LinkEntry;
use crate::seo::types::entries::meta::MetaEntry;
use crate::seo::types::entries::open_graph::OpenGraphEntry;
use crate::seo::types::entries::twitter::TwitterEntry;

pub mod entries;

pub struct SeoArgs {
  pub meta: Option<MetaEntry>,
  pub open_graph: Option<OpenGraphEntry>,
  pub twitter: Option<TwitterEntry>,
  pub link: Option<LinkEntry>,
  pub icon: Option<Vec<IconEntry>>,
}

impl Parse for SeoArgs {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    let mut meta = None;
    let mut open_graph = None;
    let mut twitter = None;
    let mut link = None;
    let mut icon = None;

    while !input.is_empty() {
      let lookahead = input.lookahead1();
      if lookahead.peek(kw::meta) {
        input.parse::<kw::meta>()?;
        let content;
        parenthesized!(content in input);
        meta = Some(content.parse::<MetaEntry>()?);
      } else if lookahead.peek(kw::open_graph) {
        input.parse::<kw::open_graph>()?;
        let content;
        parenthesized!(content in input);
        open_graph = Some(content.parse::<OpenGraphEntry>()?);
      } else if lookahead.peek(kw::twitter) {
        input.parse::<kw::twitter>()?;
        let content;
        parenthesized!(content in input);
        twitter = Some(content.parse::<TwitterEntry>()?);
      } else if lookahead.peek(kw::link) {
        input.parse::<kw::link>()?;
        let content;
        parenthesized!(content in input);
        link = Some(content.parse::<LinkEntry>()?);
      } else if lookahead.peek(kw::icon) {
        input.parse::<kw::icon>()?;
        let content;
        parenthesized!(content in input);
        let entries = content.parse_terminated(|p| {
          let p_content;
          parenthesized!(p_content in p);
          p_content.parse::<IconEntry>()
        }, Token![,])?;
        icon = Some(entries.into_iter().collect());
      } else {
        return Err(lookahead.error());
      }
      if input.peek(Token![,]) {
        input.parse::<Token![,]>()?;
      }
    }

    Ok(Self {
      meta,
      open_graph,
      twitter,
      link,
      icon,
    })
  }
}

fn option_litstr_tokens(opt: Option<&syn::LitStr>) -> proc_macro2::TokenStream {
  match opt {
    Some(lit) => {
      let s = lit.value();
      quote! { ::std::option::Option::Some(#s.to_string()) }
    }
    None => quote! { ::std::option::Option::None },
  }
}

fn option_vec_litstr_tokens(opt: Option<&Vec<syn::LitStr>>) -> proc_macro2::TokenStream {
  match opt {
    Some(vec) => {
      let strings: Vec<String> = vec.iter().map(|l| l.value()).collect();
      quote! { ::std::option::Option::Some(::std::vec![#(#strings.to_string()),*]) }
    }
    None => quote! { ::std::option::Option::None },
  }
}

mod kw {
  use syn::custom_keyword;

  custom_keyword!(meta);
  custom_keyword!(open_graph);
  custom_keyword!(twitter);
  custom_keyword!(link);
  custom_keyword!(icon);
}
