#![feature(provide_any)]
#![feature(downcast_unchecked)]

use resource_manager::{AnyProvider, AnyProviderImpl, AnyProviderInner, ConstProvider};

fn main() {
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
