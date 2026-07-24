use proc_macro2::Ident;
use quote::quote;
use syn::{Error, LitStr, Token};
use syn::parse::{Parse, ParseStream};
use crate::seo::types::option_litstr_tokens;

#[derive(Debug, Clone)]
pub enum TwitterCardType {
  Summary,
  SummaryLargeImage,
  App,
  Player,
}

impl TwitterCardType {
  fn from(t: String) -> Option<Self> {
    match t.to_lowercase().as_str() {
      "summary" => Some(TwitterCardType::Summary),
      "summary_large_image" => Some(TwitterCardType::SummaryLargeImage),
      "app" => Some(TwitterCardType::App),
      "player" => Some(TwitterCardType::Player),
      _ => None
    }
  }
}

pub struct TwitterEntry {
  pub card: Option<LitStr>,
  pub site: Option<LitStr>,
  pub creator: Option<LitStr>,
  pub title: Option<LitStr>,
  pub description: Option<LitStr>,
  pub image: Option<LitStr>,
  pub image_alt: Option<LitStr>,
  pub image_width: Option<LitStr>,
  pub image_height: Option<LitStr>,
  pub image_type: Option<LitStr>,
}

impl Parse for TwitterEntry {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    let mut card = None;
    let mut site = None;
    let mut creator = None;
    let mut title = None;
    let mut description = None;
    let mut image = None;
    let mut image_alt = None;
    let mut image_width = None;
    let mut image_height = None;
    let mut image_type = None;

    let entries = input.parse_terminated(|input| -> syn::Result<(Ident, LitStr)> {
      let key: Ident = input.parse()?;
      input.parse::<Token![=]>()?;
      let value: LitStr = input.parse()?;
      Ok((key, value))
    }, Token![,], )?;

    for (key, value) in entries {
      match key.to_string().as_str() {
        "card" => card = Some(value),
        "site" => site = Some(value),
        "creator" => creator = Some(value),
        "title" => title = Some(value),
        "description" => description = Some(value),
        "image" => image = Some(value),
        "image_alt" => image_alt = Some(value),
        "image_width" => image_width = Some(value),
        "image_height" => image_height = Some(value),
        "image_type" => image_type = Some(value),
        rest => return Err(Error::new(key.span(), format!("Invalid key: {}", rest))),
      }
    }
    
    if card.is_some() && TwitterCardType::from(card.as_ref().unwrap().value()).is_none() {
      return Err(Error::new(card.as_ref().unwrap().span(), format!("Invalid card type: {}", card.as_ref().unwrap().value())));
    }

    Ok(Self {
      card,
      site,
      creator,
      title,
      description,
      image,
      image_alt,
      image_width,
      image_height,
      image_type,
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
    let image = option_litstr_tokens(self.image.as_ref());
    let image_alt = option_litstr_tokens(self.image_alt.as_ref());
    let image_width = option_litstr_tokens(self.image_width.as_ref());
    let image_height = option_litstr_tokens(self.image_height.as_ref());
    let image_type = option_litstr_tokens(self.image_type.as_ref());

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