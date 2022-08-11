use std::{any::Any, collections::HashMap};

pub trait AnyProviderInner {
    fn register_prefix(&mut self, prefix: String, element: Box<dyn AnyProviderInner>);
    fn register_any(&mut self, key: &str, element: Box<dyn Any>);
    fn get_any(&self, key: &str) -> Option<&dyn Any>;
}
pub trait AnyProvider: AnyProviderInner {
    fn register<T: 'static>(&mut self, key: &str, element: Box<T>) {
        self.register_any(key, element as Box<dyn Any>)
    }
    fn get<T: 'static>(&self, key: &str) -> Option<&T> {
        self.get_any(key)
            .map(|val| val.downcast_ref::<T>())
            .flatten()
    }
}
pub struct AnyProviderImpl {
    prefixes: HashMap<String, Box<dyn AnyProviderInner>>,
    contents: HashMap<String, Box<dyn Any>>,
}
impl AnyProviderImpl {
    pub fn new() -> Self {
        Self {
            prefixes: HashMap::new(),
            contents: HashMap::new(),
        }
    }
}
impl AnyProviderInner for AnyProviderImpl {
    fn register_prefix(&mut self, prefix: String, element: Box<dyn AnyProviderInner>) {
        self.prefixes.insert(prefix, element);
    }
    fn register_any(&mut self, key: &str, element: Box<dyn Any>) {
        for (prefix, provider) in self.prefixes.iter_mut() {
            if key.starts_with(prefix) {
                return provider.register_any(key, element);
            }
        }
        self.contents.insert(key.to_string(), element);
    }
    fn get_any(&self, key: &str) -> Option<&dyn Any> {
        for (prefix, provider) in self.prefixes.iter() {
            if key.starts_with(prefix) {
                return provider.get_any(key);
            }
        }
        self.contents.get(key).map(AsRef::as_ref)
    }
}
impl AnyProvider for AnyProviderImpl {}

pub struct ConstProvider<T> {
    value: T,
    prefixes: HashMap<String, Box<dyn AnyProviderInner>>,
}
impl<T> ConstProvider<T> {
    pub fn new(value: T) -> Self {
        Self {
            prefixes: HashMap::new(),
            value,
        }
    }
}
impl<T: 'static> AnyProviderInner for ConstProvider<T> {
    fn register_prefix(&mut self, prefix: String, element: Box<dyn AnyProviderInner>) {
        self.prefixes.insert(prefix, element);
    }

    fn register_any(&mut self, key: &str, element: Box<dyn Any>) {
        for (prefix, provider) in self.prefixes.iter_mut() {
            if key.starts_with(prefix) {
                return provider.register_any(key, element);
            }
        }
        // Immutable
    }

    fn get_any(&self, key: &str) -> Option<&dyn Any> {
        for (prefix, provider) in self.prefixes.iter() {
            if key.starts_with(prefix) {
                return provider.get_any(key);
            }
        }
        Some(&self.value)
    }
}
