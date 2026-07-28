use syn::{LitStr, Token};
use syn::parse::{Parse, ParseStream};

#[derive(Debug, Clone)]
pub enum TwitterCardType {
  Summary,
  SummaryLargeImage,
  App,
  Player,
}

impl TwitterCardType {
  pub fn from(t: String) -> Option<Self> {
    match t.to_lowercase().as_str() {
      "summary" => Some(TwitterCardType::Summary),
      "summary_large_image" => Some(TwitterCardType::SummaryLargeImage),
      "app" => Some(TwitterCardType::App),
      "player" => Some(TwitterCardType::Player),
      _ => None
    }
  }
}

pub struct TwImageEntry {
  pub url: Option<LitStr>,
  pub alt: Option<LitStr>,
  pub width: Option<LitStr>,
  pub height: Option<LitStr>,
  pub type_: Option<LitStr>,
}

impl Parse for TwImageEntry {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    let mut url = None;
    let mut alt = None;
    let mut width = None;
    let mut height = None;
    let mut type_ = None;

    while !input.is_empty() {
      let lookahead = input.lookahead1();
      if lookahead.peek(kw::url) {
        input.parse::<kw::url>()?;
        input.parse::<Token![=]>()?;
        url = Some(input.parse()?);
      } else if lookahead.peek(kw::alt) {
        input.parse::<kw::alt>()?;
        input.parse::<Token![=]>()?;
        alt = Some(input.parse()?);
      } else if lookahead.peek(kw::width) {
        input.parse::<kw::width>()?;
        input.parse::<Token![=]>()?;
        width = Some(input.parse()?);
      } else if lookahead.peek(kw::height) {
        input.parse::<kw::height>()?;
        input.parse::<Token![=]>()?;
        height = Some(input.parse()?);
      } else if lookahead.peek(kw::type_) {
        input.parse::<kw::type_>()?;
        input.parse::<Token![=]>()?;
        type_ = Some(input.parse()?);
      } else {
        return Err(lookahead.error());
      }

      if input.peek(Token![,]) {
        input.parse::<Token![,]>()?;
      }
    }

    Ok(Self {
      url,
      alt,
      width,
      height,
      type_,
    })
  }
}

mod kw {
  use syn::custom_keyword;

  custom_keyword!(url);
  custom_keyword!(alt);
  custom_keyword!(width);
  custom_keyword!(height);
  custom_keyword!(type_);
}