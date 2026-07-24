pub trait ObjToIter {
  fn to_iter(self) -> Vec<(String, Option<String>)>;
}