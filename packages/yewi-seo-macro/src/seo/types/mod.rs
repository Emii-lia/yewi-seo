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
        let entries = content.parse_terminated(IconEntry::parse, Token![,])?;
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
  custom_keyword!(title);
  custom_keyword!(description);
  custom_keyword!(application_name);
  custom_keyword!(author);
  custom_keyword!(generator);
  custom_keyword!(keywords);
  custom_keyword!(referrer);
  custom_keyword!(creator);
  custom_keyword!(publisher);
  custom_keyword!(robots);
  custom_keyword!(theme_color);
  custom_keyword!(viewport);
  custom_keyword!(abstract_);
  custom_keyword!(category);
  custom_keyword!(classification);
  custom_keyword!(manifest);
  custom_keyword!(canonical);
  custom_keyword!(image);
  custom_keyword!(image_width);
  custom_keyword!(image_height);
  custom_keyword!(image_alt);
  custom_keyword!(image_type);
  custom_keyword!(image_secure_url);
  custom_keyword!(url);
  custom_keyword!(site_name);
  custom_keyword!(locale);
  custom_keyword!(alternate_locale);
  custom_keyword!(audio);
  custom_keyword!(audio_secure_url);
  custom_keyword!(audio_type);
  custom_keyword!(type_);
  custom_keyword!(video);
  custom_keyword!(video_width);
  custom_keyword!(video_height);
  custom_keyword!(video_type);
  custom_keyword!(video_secure_url);
  custom_keyword!(country_name);
  custom_keyword!(determiner);
  custom_keyword!(card);
  custom_keyword!(site);
  custom_keyword!(href);
  custom_keyword!(sizes);
  custom_keyword!(rel);
  custom_keyword!(color);
}
