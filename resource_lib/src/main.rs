#![feature(ptr_metadata, type_name_of_val)]
use std::any::{type_name_of_val, Any, TypeId};

fn main() {
  let stack_foo = Foo { member: 2 };
  println!("stack {:p}", &stack_foo);

  let foo = Box::new(Foo { member: 8 });
  println!("foo {:p}", foo);
  let real_baup: &dyn Baup = foo.as_ref();
  real_baup.baup();
  println!("real_baup {:p}", real_baup);

  let targets = foo.traitcastable_from();
  let any_foo: Box<dyn Any> = foo;
  println!("any_foo {:p}", any_foo);
  let baup = trait_cast::<dyn Baup>(any_foo.as_ref(), &targets).unwrap();
  baup.baup();

  let baup = trait_cast::<Foo>(any_foo.as_ref(), &targets);
  baup.unwrap();

  // BAD BOX AS ANY
  // See very save even when source is wrong!
  assert!(trait_cast::<dyn Baup>(&any_foo, &targets).is_none());
}
