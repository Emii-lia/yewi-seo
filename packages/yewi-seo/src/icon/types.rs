use crate::traits::ObjToIter;

#[derive(Debug, Clone)]
pub struct IconProps {
  pub href: String,
  pub sizes: Option<String>,
  pub type_: Option<String>,
  pub rel: Option<String>,
  pub color: Option<String>,
}
#[derive(Debug, Clone, Default)]
pub struct SeoIconProps {
  pub icons: Vec<IconProps>,
}

impl ObjToIter for IconProps {
  fn to_iter(self) -> Vec<(String, Option<String>)> {
    vec![
      ("href".to_string(), Some(self.href)),
      ("sizes".to_string(), Some(self.sizes.unwrap_or("any".to_string()))),
      ("type".to_string(), self.type_),
      ("rel".to_string(), Some(self.rel.unwrap_or("icon".to_string()))),
      ("color".to_string(), self.color),
    ]
  }
}