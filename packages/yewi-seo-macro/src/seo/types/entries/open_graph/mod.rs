pub mod types;

use quote::quote;
use syn::{parenthesized, Error, LitStr, Token};
use syn::parse::{Parse, ParseStream};
use crate::seo::types::{option_litstr_tokens, option_vec_litstr_tokens};
use crate::seo::types::entries::open_graph::types::{OgAudioEntry, OgImageEntry, OgVideoEntry};

pub struct OpenGraphEntry {
  pub title: Option<LitStr>,
  pub description: Option<LitStr>,
  pub image: Option<OgImageEntry>,
  pub url: Option<LitStr>,
  pub type_: Option<LitStr>,
  pub site_name: Option<LitStr>,
  pub locale: Option<LitStr>,
  pub alternate_locale: Option<Vec<LitStr>>,
  pub audio: Option<OgAudioEntry>,
  pub video: Option<OgVideoEntry>,
  pub country_name: Option<LitStr>,
  pub determiner: Option<LitStr>,
}

impl Parse for OpenGraphEntry {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    let mut title = None;
    let mut description = None;
    let mut image = None;
    let mut url = None;
    let mut type_ = None;
    let mut site_name = None;
    let mut locale = None;
    let mut alternate_locale = None;
    let mut audio = None;
    let mut video = None;
    let mut country_name = None;
    let mut determiner = None;

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
      } else if lookahead.peek(kw::image) {
        input.parse::<kw::image>()?;
        if input.peek(Token![=]) {
          input.parse::<Token![=]>()?;
          let image_url: LitStr = input.parse()?;
          image = Some(OgImageEntry {
            url: Some(image_url),
            secure_url: None,
            type_: None,
            width: None,
            height: None,
            alt: None,
          });
        } else {
          let content;
          parenthesized!(content in input);
          image = Some(content.parse::<OgImageEntry>()?);
        }
      } else if lookahead.peek(kw::url) {
        input.parse::<kw::url>()?;
        input.parse::<Token![=]>()?;
        url = Some(input.parse()?);
      } else if lookahead.peek(kw::type_) {
        input.parse::<kw::type_>()?;
        input.parse::<Token![=]>()?;
        type_ = Some(input.parse()?);
      } else if lookahead.peek(kw::site_name) {
        input.parse::<kw::site_name>()?;
        input.parse::<Token![=]>()?;
        site_name = Some(input.parse()?);
      } else if lookahead.peek(kw::locale) {
        input.parse::<kw::locale>()?;
        input.parse::<Token![=]>()?;
        locale = Some(input.parse()?);
      } else if lookahead.peek(kw::alternate_locale) {
        input.parse::<kw::alternate_locale>()?;
        let content;
        parenthesized!(content in input);
        alternate_locale = Some(content.parse_terminated(|lc| -> syn::Result<LitStr> {
          lc.parse::<LitStr>()
        }, Token![,])?.into_iter().collect());
      } else if lookahead.peek(kw::audio) {
        input.parse::<kw::audio>()?;
        let content;
        parenthesized!(content in input);
        let entries = content.parse_terminated(OgAudioEntry::parse, Token![,])?;
        audio = Some(entries.into_iter().next().ok_or_else(|| Error::new(content.span(), "Expected at least one audio entry"))?);
      } else if lookahead.peek(kw::video) {
        input.parse::<kw::video>()?;
        let content;
        parenthesized!(content in input);
        let entries = content.parse_terminated(OgVideoEntry::parse, Token![,])?;
        video = Some(entries.into_iter().next().ok_or_else(|| Error::new(content.span(), "Expected at least one video entry"))?);
      } else if lookahead.peek(kw::country_name) {
        input.parse::<kw::country_name>()?;
        input.parse::<Token![=]>()?;
        country_name = Some(input.parse()?);
      } else if lookahead.peek(kw::determiner) {
        input.parse::<kw::determiner>()?;
        input.parse::<Token![=]>()?;
        determiner = Some(input.parse()?);
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
      image,
      url,
      type_,
      site_name,
      locale,
      alternate_locale,
      audio,
      video,
      country_name,
      determiner,
    })
  }
}

impl OpenGraphEntry {
  pub fn build_open_graph_tokens(&self) -> proc_macro2::TokenStream {
    let title = option_litstr_tokens(self.title.as_ref());
    let description = option_litstr_tokens(self.description.as_ref());
    let image = option_litstr_tokens(self.image.as_ref().and_then(|image_entry| image_entry.url.as_ref()));
    let image_width = option_litstr_tokens(self.image.as_ref().and_then(|img| img.width.as_ref()));
    let image_height = option_litstr_tokens(self.image.as_ref().and_then(|img| img.height.as_ref()));
    let image_alt = option_litstr_tokens(self.image.as_ref().and_then(|img| img.alt.as_ref()));
    let image_type = option_litstr_tokens(self.image.as_ref().and_then(|img| img.type_.as_ref()));
    let image_secure_url = option_litstr_tokens(self.image.as_ref().and_then(|img| img.secure_url.as_ref()));
    let url = option_litstr_tokens(self.url.as_ref());
    let type_ = option_litstr_tokens(self.type_.as_ref());
    let site_name = option_litstr_tokens(self.site_name.as_ref());
    let locale = option_litstr_tokens(self.locale.as_ref());
    let alternate_locale = option_vec_litstr_tokens(self.alternate_locale.as_ref());
    let audio = option_litstr_tokens(self.audio.as_ref().and_then(|audio_entry| audio_entry.url.as_ref()));
    let audio_secure_url = option_litstr_tokens(self.audio.as_ref().and_then(|audio| audio.secure_url.as_ref()));
    let audio_type = option_litstr_tokens(self.audio.as_ref().and_then(|audio| audio.type_.as_ref()));
    let video = option_litstr_tokens(self.video.as_ref().and_then(|video| video.url.as_ref()));
    let video_width = option_litstr_tokens(self.video.as_ref().and_then(|video| video.width.as_ref()));
    let video_height = option_litstr_tokens(self.video.as_ref().and_then(|video| video.height.as_ref()));
    let video_type = option_litstr_tokens(self.video.as_ref().and_then(|video| video.type_.as_ref()));
    let video_secure_url = option_litstr_tokens(self.video.as_ref().and_then(|video| video.secure_url.as_ref()));
    let country_name = option_litstr_tokens(self.country_name.as_ref());
    let determiner = option_litstr_tokens(self.determiner.as_ref());

    quote! {
        ::yewi_seo::apply_seo_open_graph(::yewi_seo::OpenGraphProps {
            title: #title,
            description: #description,
            image: #image,
            image_width: #image_width,
            image_height: #image_height,
            image_alt: #image_alt,
            image_type: #image_type,
            image_secure_url: #image_secure_url,
            url: #url,
            type_: #type_,
            site_name: #site_name,
            locale: #locale,
            alternate_locale: #alternate_locale,
            audio: #audio,
            audio_secure_url: #audio_secure_url,
            audio_type: #audio_type,
            video: #video,
            video_width: #video_width,
            video_height: #video_height,
            video_type: #video_type,
            video_secure_url: #video_secure_url,
            country_name: #country_name,
            determiner: #determiner,
        })
    }
  }
}

mod kw {
  use syn::custom_keyword;

  custom_keyword!(title);
  custom_keyword!(description);
  custom_keyword!(image);
  custom_keyword!(image_width);
  custom_keyword!(image_height);
  custom_keyword!(image_alt);
  custom_keyword!(image_type);
  custom_keyword!(image_secure_url);
  custom_keyword!(url);
  custom_keyword!(type_);
  custom_keyword!(site_name);
  custom_keyword!(locale);
  custom_keyword!(alternate_locale);
  custom_keyword!(audio);
  custom_keyword!(video);
  custom_keyword!(country_name);
  custom_keyword!(determiner);
}