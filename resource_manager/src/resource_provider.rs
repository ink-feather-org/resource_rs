use crate::ResourceStrPart;
use std::{error::Error, marker::StructuralEq};
use trait_cast_rs::TraitcastableAny;

pub trait ResourceType: Eq + StructuralEq + Copy {
  type Resource: TraitcastableAny;
  fn default_resource(&self) -> &Self::Resource;
}

pub trait RpResourceHandle {
  type Resource: TraitcastableAny;
  type ResourceType: ResourceType;
  fn size_bytes(&self) -> Option<usize>;
  fn load(&self) -> Result<&Self::Resource, &dyn Error>;
  fn load_or_default(&self) -> &Self::Resource;
  fn get(&self) -> Option<&Self::Resource>;
  fn unload(&self);
  fn get_type(&self) -> Self::ResourceType;
}

/// A stable id that uniquely identifies a resource.
/// Must be unique per `ResourceProvider` instance implementation.
#[derive(Debug, Clone, Copy)]
pub struct RpId {
  rp_id: u128,
}

pub trait ResourceProvider<Resource: TraitcastableAny> {
  fn get_handle(&mut self, rp_id: RpId) -> Option<impl RpResourceHandle<Resource = Resource>>;
  fn get_res_strs(&self) -> impl Iterator<Item = (ResourceStrPart, RpId)>;
}
