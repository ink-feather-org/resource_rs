use crate::{ResourceIndex, ResourceStr, RpResourceHandle};
use thiserror::Error;

pub struct ResourceHandle {}
impl ResourceHandle {
  fn res_str(&self) -> ResourceStr {
    todo!()
  }
  fn res_id(&self) -> ResourceIndex {
    todo!()
  }
}
impl RpResourceHandle for ResourceHandle {}

pub trait ResourceProvider {
  fn get_handle<'a>(&mut self, key: &TrimmedResourceString<'a>) -> Option<ResourceHandle>;
  fn get_res_strs(&self) -> impl Iterator<Item = TrimmedResourceString>;
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
    ($manager_name:ident: {$($name:ident: $type:ty),*$(,)?}) => {
        use ::std::{collections::HashMap, any::{type_name, TypeId}};

        #[derive(Debug)]
        pub struct $manager_name {
            registrar: HashMap<String, $crate::ResourceIndex>,
            $($name: Vec<$type>,)*
        }
        impl $manager_name {
            pub fn new() -> Self {
                Self {
                    registrar: HashMap::new(),
                    $($name: Vec::new(),)*
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
                        let index = self.$name.len();
                        let re_idx = $crate::ResourceIndex::new::<$type>(index);
                        self.registrar.insert(key, re_idx);

                        self.$name.push(resource);
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
                        Ok(self.$name.get(index.index()).unwrap())
                    } else {
                        Err($crate::GetError::WrongType{stored_type: Self::get_type_name(index.type_id()),
                            expected_type: type_name::<$type>()})
                    }
                }
            }
        )*
    }
}
