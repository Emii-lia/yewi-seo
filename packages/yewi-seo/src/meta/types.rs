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

impl ReferrerPolicy {
  pub fn to_string(self) -> String {
    match self {
      ReferrerPolicy::NoReferrer => "no-referrer".to_string(),
      ReferrerPolicy::NoReferrerWhenDowngrade => "no-referrer-when-downgrade".to_string(),
      ReferrerPolicy::Origin => "origin".to_string(),
      ReferrerPolicy::OriginWhenCrossOrigin => "origin-when-cross-origin".to_string(),
      ReferrerPolicy::SameOrigin => "same-origin".to_string(),
      ReferrerPolicy::StrictOriginWhenCrossOrigin => "strict-origin-when-cross-origin".to_string(),
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