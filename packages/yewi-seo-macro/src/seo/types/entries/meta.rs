use proc_macro2::Ident;
use quote::quote;
use syn::{Error, LitStr};
use syn::parse::{Parse, ParseStream};
use crate::seo::types::option_litstr_tokens;

pub struct MetaEntry {
  pub title: Option<LitStr>,
  pub description: Option<LitStr>,
  pub application_name: Option<LitStr>,
  pub author: Option<LitStr>,
  pub generator: Option<LitStr>,
  pub keywords: Option<Vec<LitStr>>,
  pub referrer: Option<LitStr>,
  pub creator: Option<LitStr>,
  pub publisher: Option<LitStr>,
  pub robots: Option<LitStr>,
  pub theme_color: Option<LitStr>,
  pub viewport: Option<LitStr>,
  pub abstract_: Option<LitStr>,
  pub category: Option<LitStr>,
  pub classification: Option<LitStr>,
}

impl Parse for MetaEntry {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    let mut title = None;
    let mut description = None;
    let mut application_name = None;
    let mut author = None;
    let mut generator = None;
    let mut keywords = None;
    let mut referrer = None;
    let mut creator = None;
    let mut publisher = None;
    let mut robots = None;
    let mut theme_color = None;
    let mut viewport = None;
    let mut abstract_ = None;
    let mut category = None;
    let mut classification = None;

    let entries = input.parse_terminated(|inp| -> syn::Result<(Ident, LitStr)> {
      let key: Ident = inp.parse()?;
      inp.parse::<syn::Token![=]>()?;
      let value: LitStr = inp.parse()?;
      Ok((key, value))
    }, syn::Token![,])?;

    for (key, value) in entries {
      match key.to_string().as_str() {
        "title" => title = Some(value),
        "description" => description = Some(value),
        "application-name" => application_name = Some(value),
        "author" => author = Some(value),
        "generator" => generator = Some(value),
        "keywords" => keywords = Some(vec![value]),
        "referrer" => referrer = Some(value),
        "creator" => creator = Some(value),
        "publisher" => publisher = Some(value),
        "robots" => robots = Some(value),
        "theme_color" => theme_color = Some(value),
        "viewport" => viewport = Some(value),
        "abstract_" => abstract_ = Some(value),
        "category" => category = Some(value),
        "classification" => classification = Some(value),
        rest => return Err(Error::new(key.span(), format!("Invalid key: {}", rest)))
      }
    }
    
    Ok(Self {
      title,
      description,
      application_name,
      author,
      generator,
      keywords,
      referrer,
      creator,
      publisher,
      robots,
      theme_color,
      viewport,
      abstract_,
      category,
      classification,
    })
  }
}

impl MetaEntry {
  pub fn build_seo_meta_tokens(&self) -> proc_macro2::TokenStream {
    let title = option_litstr_tokens(self.title.as_ref());
    let description = option_litstr_tokens(self.description.as_ref());
    let application_name = option_litstr_tokens(self.application_name.as_ref());
    let author = option_litstr_tokens(self.author.as_ref());
    let generator = option_litstr_tokens(self.generator.as_ref());
    let keywords = match self.keywords.as_ref(){
      Some(vec) => {
        let joined = vec.iter().map(|l| l.value()).collect::<Vec<_>>().join(", ");
        quote! { ::std::option::Option::Some(#joined.to_string()) }
      }
      None => quote! { ::std::option::Option::None },
    };
    let referrer = option_litstr_tokens(self.referrer.as_ref());
    let creator = option_litstr_tokens(self.creator.as_ref());
    let publisher = option_litstr_tokens(self.publisher.as_ref());
    let robots = option_litstr_tokens(self.robots.as_ref());
    let theme_color = option_litstr_tokens(self.theme_color.as_ref());
    let viewport = option_litstr_tokens(self.viewport.as_ref());
    let abstract_ = option_litstr_tokens(self.abstract_.as_ref());
    let category = option_litstr_tokens(self.category.as_ref());
    let classification = option_litstr_tokens(self.classification.as_ref());

    quote! {
        ::yewi_seo::apply_seo_meta(::yewi_seo::SeoMetaProps {
            title: #title,
            description: #description,
            application_name: #application_name,
            author: #author,
            generator: #generator,
            keywords: #keywords,
            referrer: #referrer,
            creator: #creator,
            publisher: #publisher,
            robots: #robots,
            theme_color: #theme_color,
            viewport: #viewport,
            abstract_: #abstract_,
            category: #category,
            classification: #classification,
        })
    }
  }
}