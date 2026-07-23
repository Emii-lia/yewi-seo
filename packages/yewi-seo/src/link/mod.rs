  use web_sys::window;
use crate::traits::ObjToIter;
use crate::utils::tag::upsert_tag;

pub mod types;

pub fn apply_link(props: types::LinkProps) {
  let Some(doc) = window().and_then(|w| w.document()) else {  return };
  
  for (attr, value) in props.to_iter() {
    let Some(value) = value else { continue };
    
    let selector = format!("link[rel='{}']", attr);
    
    upsert_tag(&doc, "link", &selector, &[("rel", &attr), ("href", &value)])
  }
}