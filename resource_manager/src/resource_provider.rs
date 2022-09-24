use crate::ResourceStrPart;
use std::error::Error;
use trait_cast_rs::TraitcastableAny;

pub trait RpResourceHandle {
  type Resource: TraitcastableAny;
  fn size_bytes(&self) -> Option<usize>;
  fn load(&self) -> Result<&Self::Resource, &dyn Error>;
  fn load_or_default(&self) -> &Self::Resource;
  fn get(&self) -> Option<&Self::Resource>;
  fn unload(&self);
}

/// A stable id that uniquely identifies a resource.
/// Must be unique per `ResourceProvider` instance implementation.
#[derive(Debug, Clone, Copy)]
pub struct RpId {
  rp_id: u64,
}

pub trait ResourceProvider<Resource: TraitcastableAny> {
  fn get_handle(&mut self, rp_id: RpId) -> Option<impl RpResourceHandle<Resource = Resource>>;
  fn get_res_strs(&self) -> impl Iterator<Item = (ResourceStrPart, RpId)>;
}
