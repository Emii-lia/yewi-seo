use proc_macro2::Ident;
use quote::quote;
use syn::{Error, LitStr, Token};
use syn::parse::{Parse, ParseStream};
use crate::seo::types::{option_litstr_tokens, option_vec_litstr_tokens};

pub struct OpenGraphEntry {
  pub title: Option<LitStr>,
  pub description: Option<LitStr>,
  pub image: Option<LitStr>,
  pub image_width: Option<LitStr>,
  pub image_height: Option<LitStr>,
  pub image_alt: Option<LitStr>,
  pub image_type: Option<LitStr>,
  pub image_secure_url: Option<LitStr>,
  pub url: Option<LitStr>,
  pub type_: Option<LitStr>,
  pub site_name: Option<LitStr>,
  pub locale: Option<LitStr>,
  pub alternate_locale: Option<Vec<LitStr>>,
  pub audio: Option<LitStr>,
  pub audio_secure_url: Option<LitStr>,
  pub audio_type: Option<LitStr>,
  pub video: Option<LitStr>,
  pub video_width: Option<u32>,
  pub video_height: Option<u32>,
  pub video_type: Option<LitStr>,
  pub video_secure_url: Option<LitStr>,
  pub country_name: Option<LitStr>,
  pub determiner: Option<LitStr>,
}

impl Parse for OpenGraphEntry {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    let mut title = None;
    let mut description = None;
    let mut image = None;
    let mut image_width = None;
    let mut image_height = None;
    let mut image_alt = None;
    let mut image_type = None;
    let mut image_secure_url = None;
    let mut url = None;
    let mut type_ = None;
    let mut site_name = None;
    let mut locale = None;
    let mut alternate_locale = None;
    let mut audio = None;
    let mut audio_secure_url = None;
    let mut audio_type = None;
    let mut video = None;
    let mut video_width = None;
    let mut video_height = None;
    let mut video_type = None;
    let mut video_secure_url = None;
    let mut country_name = None;
    let mut determiner = None;

    let entries = input.parse_terminated(|input| -> syn::Result<(Ident, LitStr)> {
      let key: Ident = input.parse()?;
      input.parse::<Token![=]>()?;
      let value: LitStr = input.parse()?;
      Ok((key, value))
    }, Token![,], )?;

    for (key, value) in entries {
      match key.to_string().as_str() {
        "title" => title = Some(value),
        "description" => description = Some(value),
        "image" => image = Some(value),
        "image_width" => image_width = Some(value),
        "image_height" => image_height = Some(value),
        "image_alt" => image_alt = Some(value),
        "image_type" => image_type = Some(value),
        "image_secure_url" => image_secure_url = Some(value),
        "url" => url = Some(value),
        "type" => type_ = Some(value),
        "site_name" => site_name = Some(value),
        "locale" => locale = Some(value),
        "alternate_locale" => alternate_locale = Some(vec![value]),
        "audio" => audio = Some(value),
        "audio_secure_url" => audio_secure_url = Some(value),
        "audio_type" => audio_type = Some(value),
        "video" => video = Some(value),
        "video_width" => video_width = Some(value.value().parse::<u32>().map_err(|_| Error::new(value.span(), "expected integer value for video_width"))?),
        "video_height" => video_height = Some(value.value().parse::<u32>().map_err(|_| Error::new(value.span(), "expected integer value for video_height"))?),
        "video_type" => video_type = Some(value),
        "video_secure_url" => video_secure_url = Some(value),
        "country_name" => country_name = Some(value),
        "determiner" => determiner = Some(value),
        rest => return Err(Error::new(key.span(), format!("Invalid key: {}", rest))),
      }
    }

    Ok(Self {
      title,
      description,
      image,
      image_width,
      image_height,
      image_alt,
      image_type,
      image_secure_url,
      url,
      type_,
      site_name,
      locale,
      alternate_locale,
      audio,
      audio_secure_url,
      audio_type,
      video,
      video_width,
      video_height,
      video_type,
      video_secure_url,
      country_name,
      determiner,
    })
  }
}

impl OpenGraphEntry {
  pub fn build_open_graph_tokens(&self) -> proc_macro2::TokenStream {
    let title = option_litstr_tokens(self.title.as_ref());
    let description = option_litstr_tokens(self.description.as_ref());
    let image = option_litstr_tokens(self.image.as_ref());
    let image_width = option_litstr_tokens(self.image_width.as_ref());
    let image_height = option_litstr_tokens(self.image_height.as_ref());
    let image_alt = option_litstr_tokens(self.image_alt.as_ref());
    let image_type = option_litstr_tokens(self.image_type.as_ref());
    let image_secure_url = option_litstr_tokens(self.image_secure_url.as_ref());
    let url = option_litstr_tokens(self.url.as_ref());
    let type_ = option_litstr_tokens(self.type_.as_ref());
    let site_name = option_litstr_tokens(self.site_name.as_ref());
    let locale = option_litstr_tokens(self.locale.as_ref());
    let alternate_locale = option_vec_litstr_tokens(self.alternate_locale.as_ref());
    let audio = option_litstr_tokens(self.audio.as_ref());
    let audio_secure_url = option_litstr_tokens(self.audio_secure_url.as_ref());
    let audio_type = option_litstr_tokens(self.audio_type.as_ref());
    let video = option_litstr_tokens(self.video.as_ref());
    let video_width = match self.video_width.as_ref(){
      Some(val) => quote! { ::std::option::Option::Some(#val) },
      None => quote! { ::std::option::Option::None },
    };
    let video_height = match self.video_height.as_ref(){
      Some(val) => quote! { ::std::option::Option::Some(#val) },
      None => quote! { ::std::option::Option::None },
    };
    let video_type = option_litstr_tokens(self.video_type.as_ref());
    let video_secure_url = option_litstr_tokens(self.video_secure_url.as_ref());
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