#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use resource_manager::{gen_res_mgr, GetResource, Number, RegisterResource, Text};
use ::std::{
    collections::HashMap,
    any::{type_name, TypeId},
};
pub struct ResourceManager {
    registrar: HashMap<String, ::resource_manager::ResourceIndex>,
    number: Vec<Number>,
    text: Vec<Text>,
}
#[automatically_derived]
impl ::core::fmt::Debug for ResourceManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field3_finish(
            f,
            "ResourceManager",
            "registrar",
            &&self.registrar,
            "number",
            &&self.number,
            "text",
            &&self.text,
        )
    }
}
impl ResourceManager {
    pub fn new() -> Self {
        Self {
            registrar: HashMap::new(),
            number: Vec::new(),
            text: Vec::new(),
        }
    }
    pub fn find(
        &self,
        key: &str,
    ) -> Result<::resource_manager::ResourceIndex, ::resource_manager::GetError> {
        if let Some(&re_idx) = self.registrar.get(key) {
            Ok(re_idx)
        } else {
            Err(::resource_manager::GetError::NotFound {
                key: key.to_string(),
            })
        }
    }
    pub fn get_type_name(type_id: TypeId) -> &'static str {
        match type_id {
            id if id == TypeId::of::<Number>() => type_name::<Number>(),
            TypeId::of::<Text>() => type_name::<Text>(),
            _ => ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                &["Unexpected type_id"],
                &[],
            )),
        }
    }
}
impl ::resource_manager::RegisterResource<Number> for ResourceManager {
    fn register(
        &mut self,
        key: String,
        resource: Number,
    ) -> Result<::resource_manager::ResourceIndex, ::resource_manager::RegisterError> {
        if let Some(re_idx) = self.registrar.get(&key) {
            Err(if re_idx.type_id() == TypeId::of::<Number>() {
                ::resource_manager::RegisterError::AlreadyExistingSameType { key }
            } else {
                ::resource_manager::RegisterError::AlreadyExistingOtherType {
                    key,
                    old_type: re_idx.type_name(),
                    new_type: type_name::<Number>(),
                }
            })
        } else {
            let index = self.number.len();
            let re_idx = ::resource_manager::ResourceIndex::new::<Number>(index);
            self.registrar.insert(key, re_idx);
            self.number.push(resource);
            Ok(re_idx)
        }
    }
}
impl ::resource_manager::GetResource<Number> for ResourceManager {
    fn get_id(&self, key: &str) -> Result<&Number, ::resource_manager::GetError> {
        let index = self.find(key)?;
        self.get(index)
    }
    fn get(
        &self,
        index: ::resource_manager::ResourceIndex,
    ) -> Result<&Number, ::resource_manager::GetError> {
        if index.type_id() == TypeId::of::<Number>() {
            Ok(self.number.get(index.index()).unwrap())
        } else {
            Err(::resource_manager::GetError::WrongType {
                stored_type: index.type_name(),
                expected_type: type_name::<Number>(),
            })
        }
    }
}
impl ::resource_manager::RegisterResource<Text> for ResourceManager {
    fn register(
        &mut self,
        key: String,
        resource: Text,
    ) -> Result<::resource_manager::ResourceIndex, ::resource_manager::RegisterError> {
        if let Some(re_idx) = self.registrar.get(&key) {
            Err(if re_idx.type_id() == TypeId::of::<Text>() {
                ::resource_manager::RegisterError::AlreadyExistingSameType { key }
            } else {
                ::resource_manager::RegisterError::AlreadyExistingOtherType {
                    key,
                    old_type: re_idx.type_name(),
                    new_type: type_name::<Text>(),
                }
            })
        } else {
            let index = self.text.len();
            let re_idx = ::resource_manager::ResourceIndex::new::<Text>(index);
            self.registrar.insert(key, re_idx);
            self.text.push(resource);
            Ok(re_idx)
        }
    }
}
impl ::resource_manager::GetResource<Text> for ResourceManager {
    fn get_id(&self, key: &str) -> Result<&Text, ::resource_manager::GetError> {
        let index = self.find(key)?;
        self.get(index)
    }
    fn get(
        &self,
        index: ::resource_manager::ResourceIndex,
    ) -> Result<&Text, ::resource_manager::GetError> {
        if index.type_id() == TypeId::of::<Text>() {
            Ok(self.text.get(index.index()).unwrap())
        } else {
            Err(::resource_manager::GetError::WrongType {
                stored_type: index.type_name(),
                expected_type: type_name::<Text>(),
            })
        }
    }
}
fn main() {
    let mut res_mgr = ResourceManager::new();
    res_mgr
        .register(String::from("number1"), Number::new(3))
        .unwrap();
    res_mgr
        .register(String::from("text1"), Text::new("Test text".to_string()))
        .unwrap();
    let num: &Number = res_mgr.get_id("number1").unwrap();
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(
            &["got: ", "\n"],
            &[::core::fmt::ArgumentV1::new_display(&num)],
        ));
    };
    let text: &Text = res_mgr.get_id("text1").unwrap();
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(
            &["got: ", "\n"],
            &[::core::fmt::ArgumentV1::new_display(&text)],
        ));
    };
    let fail_text: Result<&Text, _> = res_mgr.get_id("number1");
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(
            &["got Error: ", "\n"],
            &[::core::fmt::ArgumentV1::new_display(
                &fail_text.unwrap_err(),
            )],
        ));
    };
    let idx = res_mgr.find("text1").unwrap();
    let txt: &Text = res_mgr.get(idx).unwrap();
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(
            &["idx_txt: ", "\n"],
            &[::core::fmt::ArgumentV1::new_display(&txt)],
        ));
    };
}
