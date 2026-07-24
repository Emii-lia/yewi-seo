use crate::traits::ObjToIter;

#[derive(Debug, Clone, Default)]
pub struct LinkProps {
  pub author: Option<String>,
  pub manifest: Option<String>,
  pub canonical: Option<String>,
}

impl ObjToIter for LinkProps {
  fn to_iter(self) -> Vec<(String, Option<String>)> {
    vec![
      ("author".to_string(), self.author),
      ("manifest".to_string(), self.manifest),
      ("canonical".to_string(), self.canonical),
    ]
  }
}