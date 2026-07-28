use web_sys::window;
use crate::open_graph::types::OpenGraphProps;
use crate::traits::ObjToIter;
use crate::utils::tag::upsert_tag;

pub mod types;

pub fn apply_open_graph(props: OpenGraphProps) {
  let Some(doc) = window().and_then(|w| w.document()) else {  return };

  for (attr, value) in props.to_iter() {
    let Some(value) = value else { continue };
    let selector =
      if attr == "locale:alternate" {
        format!("meta[property='og:{}',content={}]", attr, value)
      } else {
        format!("meta[property='og:{}']", attr)
      };
    upsert_tag(&doc, "meta", &selector, &[("property", &format!("og:{}", attr)), ("content", &value)], None)
  }
}