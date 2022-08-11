use resource_manager::{gen_res_mgr, GetResource, Number, RegisterResource, Text};

gen_res_mgr!(ResourceManager: {
    number: Number,
    text: Text,
});

fn main() {
    let mut res_mgr = ResourceManager::new();

    res_mgr
        .register(String::from("number1"), Number::new(3))
        .unwrap();
    res_mgr
        .register(String::from("text1"), Text::new("Test text".to_string()))
        .unwrap();
    //res_mgr
    //    .register(String::from("number1"), Text::new("Fail text".to_string()))
    //    .unwrap();
    //dbg!(&res_mgr);

    let num: &Number = res_mgr.get_id("number1").unwrap();
    println!("got: {}", num);

    let text: &Text = res_mgr.get_id("text1").unwrap();
    println!("got: {}", text);

    let fail_text: Result<&Text, _> = res_mgr.get_id("number1");
    println!("got Error: {}", fail_text.unwrap_err());

    let idx = res_mgr.find("text1").unwrap();
    let txt: &Text = res_mgr.get(idx).unwrap();
    println!("idx_txt: {txt}");
}
