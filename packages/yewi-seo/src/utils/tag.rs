use web_sys::Document;

pub fn upsert_tag(
  doc: &Document,
  tag: &str,
  selector: &str,
  attrs: &[(&str, &str)]
) {
  let head = doc.head().expect("Document should have a head");

  let el = doc
    .query_selector(selector)
    .ok()
    .flatten()
    .unwrap_or_else(|| {
      let el = doc.create_element(tag).unwrap();
      head.append_child(&el).unwrap();
      el
    });

  for (attr, value) in attrs {
    el.set_attribute(attr, value).unwrap();
  }
}