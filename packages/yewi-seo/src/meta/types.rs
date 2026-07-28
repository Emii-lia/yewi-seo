use std::fmt::{Display, Formatter};
use crate::traits::ObjToIter;

#[derive(Debug, Clone)]
pub enum ReferrerPolicy {
  NoReferrer,
  NoReferrerWhenDowngrade,
  Origin,
  OriginWhenCrossOrigin,
  SameOrigin,
  StrictOriginWhenCrossOrigin,
}

impl Display for ReferrerPolicy {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      ReferrerPolicy::NoReferrer => write!(f, "no-referrer"),
      ReferrerPolicy::NoReferrerWhenDowngrade => write!(f, "no-referrer-when-downgrade"),
      ReferrerPolicy::Origin => write!(f, "origin"),
      ReferrerPolicy::OriginWhenCrossOrigin => write!(f, "origin-when-cross-origin"),
      ReferrerPolicy::SameOrigin => write!(f, "same-origin"),
      ReferrerPolicy::StrictOriginWhenCrossOrigin => write!(f, "strict-origin-when-cross-origin")
    }
  }
}

#[derive(Debug, Clone, Default)]
pub struct SeoMetaProps {
  pub title: Option<String>,
  pub description: Option<String>,
  pub application_name: Option<String>,
  pub author: Option<String>,
  pub generator: Option<String>,
  pub keywords: Option<String>,
  pub referrer: Option<ReferrerPolicy>,
  pub creator: Option<String>,
  pub publisher: Option<String>,
  pub robots: Option<String>,
  pub theme_color: Option<String>,
  pub viewport: Option<String>,
  pub abstract_: Option<String>,
  pub category: Option<String>,
  pub classification: Option<String>,
}

impl ObjToIter for SeoMetaProps {
  fn to_iter(self) -> Vec<(String, Option<String>)> {
    vec![
      ("title".to_string(), self.title),
      ("description".to_string(), self.description),
      ("application-name".to_string(), self.application_name),
      ("author".to_string(), self.author),
      ("generator".to_string(), self.generator),
      ("keywords".to_string(), self.keywords),
      ("referrer".to_string(), self.referrer.map(|r| r.to_string())),
      ("creator".to_string(), self.creator),
      ("publisher".to_string(), self.publisher),
      ("robots".to_string(), self.robots),
      ("theme-color".to_string(), self.theme_color),
      ("viewport".to_string(), self.viewport),
      ("abstract".to_string(), self.abstract_),
      ("category".to_string(), self.category),
      ("classification".to_string(), self.classification),
    ]
  }
}