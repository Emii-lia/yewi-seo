use crate::traits::ObjToIter;

#[derive(Debug, Clone, Default)]
pub struct TwitterCardProps {
  pub card: Option<String>,
  pub site: Option<String>,
  pub creator: Option<String>,
  pub title: Option<String>,
  pub description: Option<String>,
  pub image: Option<String>,
  pub image_alt: Option<String>,
  pub image_width: Option<String>,
  pub image_height: Option<String>,
  pub image_type: Option<String>,
}

impl ObjToIter for TwitterCardProps {
  fn to_iter(self) -> Vec<(String, Option<String>)> {
    vec![
      ("card".to_string(), self.card),
      ("site".to_string(), self.site),
      ("creator".to_string(), self.creator),
      ("title".to_string(), self.title),
      ("description".to_string(), self.description),
      ("image".to_string(), self.image),
      ("image:alt".to_string(), self.image_alt),
      ("image:width".to_string(), self.image_width),
      ("image:height".to_string(), self.image_height),
      ("image:type".to_string(), self.image_type),
    ]
  }
}