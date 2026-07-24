use web_sys::window;
use crate::icon::types::SeoIconProps;
use crate::utils::tag::upsert_tag;

pub mod types;

pub fn apply_icons(props: SeoIconProps) {
  let Some(doc) = window().and_then(|w| w.document()) else {  return };
  
  for icon in props.icons {
    let selector = format!("link[rel='{}', href='{}', sizes={}]", icon.rel.clone().unwrap_or_default(), icon.href.clone(), icon.sizes.clone().unwrap_or_default());
    upsert_tag(&doc, "link", &selector, &[
      ("rel", &icon.rel.unwrap_or("icon".to_string())),
      ("href", &icon.href),
      ("sizes", &icon.sizes.unwrap_or("any".to_string())),
      ("type", &icon.type_.unwrap_or_default()),
      ("color", &icon.color.unwrap_or_default()),
    ]);
  }
}