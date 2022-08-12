//#![allow(incomplete_features)] // Maybe use min_specialization
#![feature(min_specialization)]

use std::fmt::Debug;

trait Tr<T: ?Sized> {
    fn x(v: &T);
}
struct S;

impl<T: Debug> Tr<T> for S {
    fn x(v: &T) {
        println!("Sized({}): {:?}", std::any::type_name::<T>(), v)
    }
}
impl<T: Debug + ?Sized> Tr<T> for S {
    default fn x(v: &T) {
        println!("Unsized({}): {:?}", std::any::type_name::<T>(), v)
    }
}
use resource_manager::{
    gen_res_mgr, AnyProvider, AnyProviderImpl, AnyProviderInner, ConstProvider, GetResource,
    Number, RegisterResource, Text,
};

gen_res_mgr!(ResourceManager: { Number, Text,});

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

    let int: &i32 = &5;
    S::x(int);
    let str: &[u8] = &[1, 4, 7];
    S::x(str);

    let mut prov = AnyProviderImpl::new();
    let mut test = Box::from(ConstProvider::new(69u8));
    test.register_prefix("val".to_string(), Box::from(AnyProviderImpl::new()));
    prov.register_prefix("Test".to_string(), test);
    prov.register("val1", Box::new(5u32));
    prov.register("val2", Box::new(5u16));
    prov.register("test1", Box::new(5u16));
    prov.register("testval4", Box::new(420u16));

    let load: &u16 = prov.get("val2").unwrap();
    println!("{load}");

    let load: &u16 = prov.get("testval4").unwrap();
    println!("{load}");

    let load: &u8 = prov.get("Test1").unwrap();
    println!("{load}");
}
