use web_sys::window;
use crate::meta::types::SeoMetaProps;
use crate::traits::ObjToIter;
use crate::utils::tag::upsert_tag;

pub mod types;

pub fn apply_seo_meta(props: SeoMetaProps) {
  let Some(doc) = window().and_then(|w| w.document()) else {  return };

  if let Some(title) = &props.title {
    doc.set_title(&title);
  }

  for (attr, value) in props.to_iter() {
    let Some(value) = value else { continue };
    if attr == "title" { continue; }

    let selector = format!("meta[name='{}']", attr);
    upsert_tag(&doc, "meta", &selector, &[("name", &attr), ("content", &value)])
  }
}