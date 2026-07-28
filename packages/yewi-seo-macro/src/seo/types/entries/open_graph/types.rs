use syn::{LitInt, LitStr, Token};
use syn::parse::{Parse, ParseStream};

pub struct OgAudioEntry {
  pub url: Option<LitStr>,
  pub secure_url: Option<LitStr>,
  pub type_: Option<LitStr>,
}
impl Parse for OgAudioEntry {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    let mut url = None;
    let mut secure_url = None;
    let mut type_ = None;

    while !input.is_empty() {
      let lookahead = input.lookahead1();
      if lookahead.peek(kw::url) {
        input.parse::<kw::url>()?;
        input.parse::<Token![=]>()?;
        url = Some(input.parse()?);
      } else if lookahead.peek(kw::secure_url) {
        input.parse::<kw::secure_url>()?;
        input.parse::<Token![=]>()?;
        secure_url = Some(input.parse()?);
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
      secure_url,
      type_,
    })
  }
}

pub struct OgVideoEntry {
  pub url: Option<LitStr>,
  pub secure_url: Option<LitStr>,
  pub type_: Option<LitStr>,
  pub width: Option<LitStr>,
  pub height: Option<LitStr>,
}

impl Parse for OgVideoEntry {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    let mut url = None;
    let mut secure_url = None;
    let mut type_ = None;
    let mut width = None;
    let mut height = None;

    while !input.is_empty() {
      let lookahead = input.lookahead1();
      if lookahead.peek(kw::url) {
        input.parse::<kw::url>()?;
        input.parse::<Token![=]>()?;
        url = Some(input.parse()?);
      } else if lookahead.peek(kw::secure_url) {
        input.parse::<kw::secure_url>()?;
        input.parse::<Token![=]>()?;
        secure_url = Some(input.parse()?);
      } else if lookahead.peek(kw::type_) {
        input.parse::<kw::type_>()?;
        input.parse::<Token![=]>()?;
        type_ = Some(input.parse()?);
      } else if lookahead.peek(kw::width) {
        input.parse::<kw::width>()?;
        input.parse::<Token![=]>()?;
        if input.peek(LitStr) {
          width = Some(input.parse()?);
        } else if input.peek(LitInt) {
          let int: LitInt = input.parse()?;
          width = Some(LitStr::new(int.base10_digits(), int.span()));
        } else {
          return Err(syn::Error::new(input.span(), "Expected a string or integer literal for width"));
        }
      } else if lookahead.peek(kw::height) {
        input.parse::<kw::height>()?;
        input.parse::<Token![=]>()?;
        if input.peek(LitInt) {
          let int: LitInt = input.parse()?;
          height = Some(LitStr::new(int.base10_digits(), int.span()));
        } else if input.peek(LitStr) {
          height = Some(input.parse()?);
        } else {
          return Err(syn::Error::new(input.span(), "Expected a string or integer literal for height"));
        }
      } else {
        return Err(lookahead.error());
      }

      if input.peek(Token![,]) {
        input.parse::<Token![,]>()?;
      }
    }

    Ok(Self {
      url,
      secure_url,
      type_,
      width,
      height,
    })
  }
}

pub struct OgImageEntry {
  pub url: Option<LitStr>,
  pub secure_url: Option<LitStr>,
  pub type_: Option<LitStr>,
  pub width: Option<LitStr>,
  pub height: Option<LitStr>,
  pub alt: Option<LitStr>,
}

impl Parse for OgImageEntry {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    let mut url = None;
    let mut secure_url = None;
    let mut type_ = None;
    let mut width = None;
    let mut height = None;
    let mut alt = None;

    while !input.is_empty() {
      let lookahead = input.lookahead1();
      if lookahead.peek(kw::url) {
        input.parse::<kw::url>()?;
        input.parse::<Token![=]>()?;
        url = Some(input.parse()?);
      } else if lookahead.peek(kw::secure_url) {
        input.parse::<kw::secure_url>()?;
        input.parse::<Token![=]>()?;
        secure_url = Some(input.parse()?);
      } else if lookahead.peek(kw::type_) {
        input.parse::<kw::type_>()?;
        input.parse::<Token![=]>()?;
        type_ = Some(input.parse()?);
      } else if lookahead.peek(kw::width) {
        input.parse::<kw::width>()?;
        input.parse::<Token![=]>()?;
        if input.peek(LitStr) {
          width = Some(input.parse()?);
        } else if input.peek(LitInt) {
          let int: LitInt = input.parse()?;
          width = Some(LitStr::new(int.base10_digits(), int.span()));
        } else {
          return Err(syn::Error::new(input.span(), "Expected a string or integer literal for width"));
        }
      } else if lookahead.peek(kw::height) {
        input.parse::<kw::height>()?;
        input.parse::<Token![=]>()?;
        if input.peek(LitStr) {
          height = Some(input.parse()?);
        } else if input.peek(LitInt) {
          let int: LitInt = input.parse()?;
          height = Some(LitStr::new(int.base10_digits(), int.span()));
        } else {
          return Err(syn::Error::new(input.span(), "Expected a string literal for height"));
        }
      } else if lookahead.peek(kw::alt) {
        input.parse::<kw::alt>()?;
        input.parse::<Token![=]>()?;
        alt = Some(input.parse()?);
      } else {
        return Err(lookahead.error());
      }

      if input.peek(Token![,]) {
        input.parse::<Token![,]>()?;
      }
    }

    Ok(Self {
      url,
      secure_url,
      type_,
      width,
      height,
      alt,
    })
  }
}

mod kw {
  use syn::custom_keyword;

  custom_keyword!(url);
  custom_keyword!(secure_url);
  custom_keyword!(type_);
  custom_keyword!(width);
  custom_keyword!(height);
  custom_keyword!(alt);
}