use crate::{ResourceStrPart, ResourceString};

#[test]
fn resource_string() {
  let a = ".abc.tk".to_string();
  let mut res = ResourceString::try_from(a.as_str()).unwrap();
  {
    let b = "def".to_string();
    res
      .push_str(ResourceStrPart::new(b.as_str()).unwrap())
      .unwrap();
  }
  //res.build();
}
