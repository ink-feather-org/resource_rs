mod number;
pub use number::Number;

mod text;
use std::any::TypeId;
pub use text::Text;
use thiserror::Error;

mod any_prov;
pub use any_prov::*;

#[derive(Debug, PartialEq, Eq, Error)]
pub enum RegisterError {
    #[error("Resource with key {key} was already registered with the same type")]
    AlreadyExistingSameType { key: String },
    #[error(
        "Resource with key {key} was already registered: old_type: {old_type} new_type: {new_type}"
    )]
    AlreadyExistingOtherType {
        key: String,
        old_type: &'static str,
        new_type: &'static str,
    },
}
#[derive(Debug, Clone, Copy)]
pub struct ResourceIndex {
    index: usize,
    type_id: TypeId,
}
impl ResourceIndex {
    #[doc(hidden)]
    pub fn new<T: 'static>(index: usize) -> Self {
        ResourceIndex {
            index,
            type_id: TypeId::of::<T>(),
        }
    }
    pub fn index(&self) -> usize {
        self.index
    }
    pub fn type_id(&self) -> TypeId {
        self.type_id
    }
}
pub trait RegisterResource<T> {
    fn register(&mut self, key: String, image: T) -> Result<ResourceIndex, RegisterError>;
}
#[derive(Debug, PartialEq, Eq, Error)]
pub enum GetError {
    #[error("Resource with key {key} was not found")]
    NotFound { key: String },
    #[error("Resource has unexpected type found: {stored_type} expected: {expected_type}")]
    WrongType {
        stored_type: &'static str,
        expected_type: &'static str,
    },
}
pub trait GetResource<T> {
    fn get_id(&self, key: &str) -> Result<&T, GetError>;
    fn get(&self, key: ResourceIndex) -> Result<&T, GetError>;
}

#[macro_export]
macro_rules! gen_res_mgr {
    ($manager_name:ident: {$($type:ty),*$(,)?}) => {
        use ::std::{collections::HashMap, any::{type_name, TypeId}};
        use ::paste::paste;
        paste!{
            #[derive(Debug)]
            pub struct $manager_name {
                registrar: HashMap<String, $crate::ResourceIndex>,
                $([<$type _collection>]: Vec<$type>,)*
            }
            impl $manager_name {
                pub fn new() -> Self {
                    Self {
                        registrar: HashMap::new(),
                        $([<$type _collection>]: Vec::new(),)*
                    }
                }
                pub fn find(&self, key: &str) -> Result<$crate::ResourceIndex, $crate::GetError> {
                    if let Some(&re_idx) = self.registrar.get(key) {
                        Ok(re_idx)
                    } else {
                        Err($crate::GetError::NotFound{key: key.to_string()})
                    }
                }
                pub fn get_type_name(type_id: TypeId) -> &'static str {
                    $(
                        if type_id == TypeId::of::<$type>() {
                            type_name::<$type>()
                        } else
                    )*
                    {
                        panic!("Unexpected type_id")
                    }
                }
            }
            $(
                impl $crate::RegisterResource<$type> for $manager_name {
                    fn register(&mut self, key: String, resource: $type) -> Result<$crate::ResourceIndex, $crate::RegisterError> {
                        if let Some(re_idx) = self.registrar.get(&key) {
                            Err(if re_idx.type_id() == TypeId::of::<$type>() {
                                $crate::RegisterError::AlreadyExistingSameType{key} // Maybe overwrite existing value
                            } else {
                                $crate::RegisterError::AlreadyExistingOtherType{key, old_type: Self::get_type_name(re_idx.type_id()), new_type: type_name::<$type>()}
                            })
                        } else {
                            let index = self.[<$type _collection>].len();
                            let re_idx = $crate::ResourceIndex::new::<$type>(index);
                            self.registrar.insert(key, re_idx);

                            self.[<$type _collection>].push(resource);
                            Ok(re_idx)
                        }
                    }
                }
                impl $crate::GetResource<$type> for $manager_name {
                    fn get_id(&self, key: &str) -> Result<&$type, $crate::GetError> {
                        let index = self.find(key)?;
                        self.get(index)
                    }
                    fn get(&self, index: $crate::ResourceIndex) -> Result<&$type, $crate::GetError> {

                        if index.type_id() == TypeId::of::<$type>() {
                            Ok(self.[<$type _collection>].get(index.index()).unwrap())
                        } else {
                            Err($crate::GetError::WrongType{stored_type: Self::get_type_name(index.type_id()),
                                expected_type: type_name::<$type>()})
                        }
                    }
                }
            )*
        }
    }
}
