use quote::quote;
use syn::{LitStr, Token};
use syn::parse::{Parse, ParseStream};
use crate::seo::types::option_litstr_tokens;

pub struct IconEntry {
  pub href: LitStr,
  pub sizes: Option<LitStr>,
  pub type_: Option<LitStr>,
  pub rel: Option<LitStr>,
  pub color: Option<LitStr>,
}

impl Parse for IconEntry {
  fn parse(input: ParseStream) -> syn::Result<Self> {

    let mut href = None;
    let mut sizes = None;
    let mut type_ = None;
    let mut rel = None;
    let mut color = None;

    while !input.is_empty() {
      let lookahead = input.lookahead1();
      if lookahead.peek(kw::href) {
        input.parse::<kw::href>()?;
        input.parse::<Token![=]>()?;
        href = input.parse()?;
      } else if lookahead.peek(kw::sizes) {
        input.parse::<kw::sizes>()?;
        input.parse::<Token![=]>()?;
        sizes = Some(input.parse()?);
      } else if lookahead.peek(kw::type_) {
        input.parse::<kw::type_>()?;
        input.parse::<Token![=]>()?;
        type_ = Some(input.parse()?);
      } else if lookahead.peek(kw::rel) {
        input.parse::<kw::rel>()?;
        input.parse::<Token![=]>()?;
        rel = Some(input.parse()?);
      } else if lookahead.peek(kw::color) {
        input.parse::<kw::color>()?;
        input.parse::<Token![=]>()?;
        color = Some(input.parse()?);
      } else {
        return Err(lookahead.error());
      }

      if input.peek(Token![,]) {
        input.parse::<Token![,]>()?;
      }
    }

    Ok(Self {
      href: href.unwrap(),
      sizes,
      type_,
      rel,
      color,
    })
  }
}

impl IconEntry {
  pub fn icon_entry_to_tokens(&self) -> proc_macro2::TokenStream {
    if self.href.value().is_empty() {
      return quote! {
        compile_error!("IconEntry must have a href value");
      }
    }
    let href = self.href.value();
    let sizes = option_litstr_tokens(self.sizes.as_ref());
    let type_ = option_litstr_tokens(self.type_.as_ref());
    let rel = option_litstr_tokens(self.rel.as_ref());
    let color = option_litstr_tokens(self.color.as_ref());
    quote! {
        ::yewi_seo::IconProps {
            href: #href.to_string(),
            sizes: #sizes,
            type_: #type_,
            rel: #rel,
            color: #color,
        }
    }
  }
  
  pub fn build_icon_tokens(icon: Self) -> proc_macro2::TokenStream {
    let icon = icon.icon_entry_to_tokens();
    quote! {
      ::yewi_seo::apply_icons(::yewi_seo::SeoIconProps {
          icons: ::std::vec![#icon],
      })
    }
  }

  pub fn build_icons_tokens(icons: Vec<Self>) -> proc_macro2::TokenStream {
    let icon_tokens: Vec<proc_macro2::TokenStream> = icons
      .into_iter()
      .map(|icon| icon.icon_entry_to_tokens())
      .collect();

    quote! {
        ::yewi_seo::apply_icons(::yewi_seo::SeoIconProps {
            icons: ::std::vec![#(#icon_tokens),*],
        })
    }
  }
}

mod kw {
  use syn::custom_keyword;

  custom_keyword!(href);
  custom_keyword!(rel);
  custom_keyword!(sizes);
  custom_keyword!(type_);
  custom_keyword!(color);
}