pub mod types;

use quote::quote;
use syn::{Error, LitStr, Token};
use syn::parse::{Parse, ParseStream};
use crate::seo::types::entries::twitter::types::{TwImageEntry, TwitterCardType};
use crate::seo::types::option_litstr_tokens;


pub struct TwitterEntry {
  pub card: Option<LitStr>,
  pub site: Option<LitStr>,
  pub creator: Option<LitStr>,
  pub title: Option<LitStr>,
  pub description: Option<LitStr>,
  pub image: Option<TwImageEntry>,
}

impl Parse for TwitterEntry {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    let mut card: Option<LitStr> = None;
    let mut site = None;
    let mut creator = None;
    let mut title = None;
    let mut description = None;
    let mut image = None;

    while !input.is_empty() {
      let lookahead = input.lookahead1();
      if lookahead.peek(kw::card) {
        input.parse::<kw::card>()?;
        input.parse::<syn::Token![=]>()?;
        card = Some(input.parse()?);
      } else if lookahead.peek(kw::site) {
        input.parse::<kw::site>()?;
        input.parse::<syn::Token![=]>()?;
        site = Some(input.parse()?);
      } else if lookahead.peek(kw::creator) {
        input.parse::<kw::creator>()?;
        input.parse::<syn::Token![=]>()?;
        creator = Some(input.parse()?);
      } else if lookahead.peek(kw::title) {
        input.parse::<kw::title>()?;
        input.parse::<syn::Token![=]>()?;
        title = Some(input.parse()?);
      } else if lookahead.peek(kw::description) {
        input.parse::<kw::description>()?;
        input.parse::<syn::Token![=]>()?;
        description = Some(input.parse()?);
      } else if lookahead.peek(kw::image) {
        input.parse::<kw::image>()?;
        if input.peek(Token![=]) {
          input.parse::<syn::Token![=]>()?;
          let url = input.parse()?;
          image = Some(TwImageEntry {
            url: Some(url),
            alt: None,
            width: None,
            height: None,
            type_: None,
          });
        } else {
          let content;
          syn::parenthesized!(content in input);
          image = Some(content.parse::<TwImageEntry>()?);
        }
      } else {
        return Err(lookahead.error());
      }

      if input.peek(syn::Token![,]) {
        input.parse::<syn::Token![,]>()?;
      }
    }

    if let Some(card) = card.clone() && TwitterCardType::from(card.value()).is_none() {
      return Err(Error::new(card.span(), format!("Invalid card type: {}", card.value())));
    }

    Ok(Self {
      card,
      site,
      creator,
      title,
      description,
      image,
    })
  }
}

impl TwitterEntry {
  pub fn build_twitter_tokens(&self) -> proc_macro2::TokenStream {
    let card = option_litstr_tokens(self.card.as_ref());
    let site = option_litstr_tokens(self.site.as_ref());
    let creator = option_litstr_tokens(self.creator.as_ref());
    let title = option_litstr_tokens(self.title.as_ref());
    let description = option_litstr_tokens(self.description.as_ref());
    let image = option_litstr_tokens(self.image.as_ref().and_then(|image| image.url.as_ref()));
    let image_alt = option_litstr_tokens(self.image.as_ref().and_then(|image| image.alt.as_ref()));
    let image_width = option_litstr_tokens(self.image.as_ref().and_then(|image| image.width.as_ref()));
    let image_height = option_litstr_tokens(self.image.as_ref().and_then(|image| image.height.as_ref()));
    let image_type = option_litstr_tokens(self.image.as_ref().and_then(|image| image.type_.as_ref()));

    quote! {
        ::yewi_seo::apply_seo_twitter_card(::yewi_seo::TwitterCardProps {
            card: #card,
            site: #site,
            creator: #creator,
            title: #title,
            description: #description,
            image: #image,
            image_alt: #image_alt,
            image_width: #image_width,
            image_height: #image_height,
            image_type: #image_type,
        })
    }
  }
}

mod kw {
  use syn::custom_keyword;

  custom_keyword!(card);
  custom_keyword!(site);
  custom_keyword!(creator);
  custom_keyword!(title);
  custom_keyword!(description);
  custom_keyword!(image);
}