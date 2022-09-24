use super::{gen_res_mgr, GetResource, RegisterResource};

mod resource_mngt_test {
  use std::any::Any;

  pub use super::*;

  gen_res_mgr!(ResourceManager: {
      nu16: u16,
      nString: String,
      bString: String,
  });

  trait ResProvider {
    fn try_load(&self, sid: String) -> Box<dyn Any>;
  }

  #[test]
  fn main() {
    let mut res_mgr = ResourceManager::new();

    res_mgr.register_provider("prefix", FileProvider::new("/overlay1"));
    res_mgr.register_provider("prefix", FileProvider::new("/overlay2"));

    res_mgr
      .register(String::from("prefix::dir::dir2::test.ogg"), 12)
      .unwrap();
    res_mgr.register(String::from("number1"), 12).unwrap();
    res_mgr
      .register(String::from("text1"), "Test text".to_string())
      .unwrap();
    //res_mgr
    //    .register(String::from("number1"), Text::new("Fail text".to_string()))
    //    .unwrap();
    //dbg!(&res_mgr);

    let num: &u16 = res_mgr.get_id("number1").unwrap();
    println!("got: {}", num);

    let text: &String = res_mgr.get_id("text1").unwrap();
    println!("got: {}", text);

    let fail_text: Result<&String, _> = res_mgr.get_id("number1");
    println!("got Error: {}", fail_text.unwrap_err());

    let idx = res_mgr.find("text1").unwrap();
    let txt: &String = res_mgr.get(idx).unwrap();
    println!("idx_txt: {txt}");
  }
}
