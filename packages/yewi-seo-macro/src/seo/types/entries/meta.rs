use quote::quote;
use syn::{parenthesized, LitStr, Token};
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

    while !input.is_empty() {
      let lookahead = input.lookahead1();
      if lookahead.peek(kw::title) {
        input.parse::<kw::title>()?;
        input.parse::<Token![=]>()?;
        title = Some(input.parse()?);
      } else if lookahead.peek(kw::description) {
        input.parse::<kw::description>()?;
        input.parse::<Token![=]>()?;
        description = Some(input.parse()?);
      } else if lookahead.peek(kw::application_name) {
        input.parse::<kw::application_name>()?;
        input.parse::<Token![=]>()?;
        application_name = Some(input.parse()?);
      } else if lookahead.peek(kw::author) {
        input.parse::<kw::author>()?;
        input.parse::<Token![=]>()?;
        author = Some(input.parse()?);
      } else if lookahead.peek(kw::generator) {
        input.parse::<kw::generator>()?;
        input.parse::<Token![=]>()?;
        generator = Some(input.parse()?);
      } else if lookahead.peek(kw::keywords) {
        input.parse::<kw::keywords>()?;
        if input.peek(Token![=]) {
          input.parse::<Token![=]>()?;
          let keys = input.parse::<LitStr>()?;
          keywords = Some(keys.value().split(',').map(|s| LitStr::new(s.trim(), keys.span())).collect());
        } else {
          let content;
          parenthesized!(content in input);
          let mut keys = Vec::new();
          while !content.is_empty() {
            let key: LitStr = content.parse()?;
            keys.push(key);
            if content.peek(Token![,]) {
              content.parse::<Token![,]>()?;
            }
          }
          keywords = Some(keys);
        }
      } else if lookahead.peek(kw::referrer) {
        input.parse::<kw::referrer>()?;
        input.parse::<Token![=]>()?;
        referrer = Some(input.parse()?);
      } else if lookahead.peek(kw::creator) {
        input.parse::<kw::creator>()?;
        input.parse::<Token![=]>()?;
        creator = Some(input.parse()?);
      } else if lookahead.peek(kw::publisher) {
        input.parse::<kw::publisher>()?;
        input.parse::<Token![=]>()?;
        publisher = Some(input.parse()?);
      } else if lookahead.peek(kw::robots) {
        input.parse::<kw::robots>()?;
        input.parse::<Token![=]>()?;
        robots = Some(input.parse()?);
      } else if lookahead.peek(kw::theme_color) {
        input.parse::<kw::theme_color>()?;
        input.parse::<Token![=]>()?;
        theme_color = Some(input.parse()?);
      } else if lookahead.peek(kw::viewport) {
        input.parse::<kw::viewport>()?;
        input.parse::<Token![=]>()?;
        viewport = Some(input.parse()?);
      } else if lookahead.peek(kw::abstract_) {
        input.parse::<kw::abstract_>()?;
        input.parse::<Token![=]>()?;
        abstract_ = Some(input.parse()?);
      } else if lookahead.peek(kw::category) {
        input.parse::<kw::category>()?;
        input.parse::<Token![=]>()?;
        category = Some(input.parse()?);
      } else if lookahead.peek(kw::classification) {
        input.parse::<kw::classification>()?;
        input.parse::<Token![=]>()?;
        classification = Some(input.parse()?);
      } else {
        return Err(lookahead.error());
      }
      if input.peek(Token![,]) {
        input.parse::<Token![,]>()?;
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

mod kw {
  use syn::custom_keyword;

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
}
