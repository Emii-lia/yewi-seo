use web_sys::window;
use crate::traits::ObjToIter;
use crate::twitter::types::TwitterCardProps;
use crate::utils::tag::upsert_tag;

pub mod types;

pub fn apply_twitter_card(props: TwitterCardProps) {
  let Some(doc) = window().and_then(|w| w.document()) else {  return };

  for (attr, value) in props.to_iter() {
    let Some(value) = value else { continue };

    let selector = format!("meta[name='twitter:{}']", attr);

    upsert_tag(&doc, "meta", &selector, &[("name", &format!("twitter:{}", attr)), ("content", &value)])
  }
}