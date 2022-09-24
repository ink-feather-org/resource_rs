use crate::RpId;
use std::fmt::{Debug, Display, Write};
use thiserror::Error;

#[derive(Debug, Clone, Copy)]
pub struct ResourceIndex {
  rp_index: u64,
  rp_id: RpId,
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum ResourceStrError {
  #[error("Invalid char {} at {}", char, at)]
  InvalidChar { at: usize, char: char },
  #[error("Empty string")]
  EmptyString,
}

/// Single level of a resource string
#[derive(Copy, Clone)]
pub struct ResourceStrPart<'a> {
  str: &'a str,
}
impl<'a> ResourceStrPart<'a> {
  pub fn is_valid_char(ch: char) -> bool {
    ch.is_alphanumeric() || ch == '_'
  }

  pub fn validate_str(str: &str) -> Result<&str, ResourceStrError> {
    if str.len() == 0 {
      return Err(ResourceStrError::EmptyString);
    }
    if let Some((at, char)) = str
      .chars()
      .enumerate()
      .find(|&(_, char)| !Self::is_valid_char(char))
    {
      return Err(ResourceStrError::InvalidChar { at, char });
    }
    Ok(str)
  }

  pub fn new(str: &'a str) -> Result<Self, ResourceStrError> {
    Self::validate_str(str)?;
    Ok(Self { str })
  }
  fn new_unchecked(str: &'a str) -> Self {
    Self { str }
  }

  fn as_str(self) -> &'a str {
    return self.str;
  }
}

// .com.assets.org.big_apple
pub struct ResourceString<'a> {
  parts: Vec<ResourceStrPart<'a>>,
}
impl<'a> ResourceString<'a> {
  const SPLITS: &[char] = &['.', '>'];

  pub fn new() -> Self {
    Self { parts: Vec::new() }
  }
  pub fn with_capacity(capacity: usize) -> Self {
    Self {
      parts: Vec::with_capacity(capacity),
    }
  }

  pub fn from_str(&'a mut self, str: &'a str) -> Result<ResourceStr<'a>, ResourceStrError> {
    self.parts.clear();
    for str in str.split(Self::SPLITS) {
      self.push_str(ResourceStrPart::new(str).map_err(|err| {
        if let ResourceStrError::InvalidChar { ref mut at, .. } = err {
          *at += self.len_chars() + 1;
        }
        err
      })?);
    }
    let depth = str.chars().rev().position(|c| c == '>').unwrap_or(0);
    Ok(ResourceStr {
      parts: &self.parts,
      depth,
    })
  }
  pub fn push_str(
    &mut self,
    str: ResourceStrPart<'a>,
  ) -> Result<ResourceStr<'a>, ResourceStrError> {
    self.parts.push(str);
    Ok(self.build())
  }
  pub fn pop_str(&mut self) -> Option<ResourceStrPart<'a>> {
    self.parts.pop()
  }
  pub fn clear(&mut self) {
    self.parts.clear()
  }

  pub fn len_chars(&self) -> usize {
    self
      .parts
      .iter()
      .map(|part| part.as_str().len())
      .sum::<usize>()
      + self.parts.len()
  }

  pub fn build(&'a self) -> ResourceStr<'a> {
    ResourceStr {
      parts: &self.parts,
      depth: 0,
    }
  }
}
impl<'a> TryFrom<&'a str> for ResourceString<'a> {
  type Error = ResourceStrError;

  fn try_from(value: &'a str) -> Result<Self, Self::Error> {
    let selv = Self::new();
    selv.from_str(value)?;
    Ok(selv)
  }
}

// >com.resource.example
// .com>resource.example // trimmed 1
#[derive(Copy, Clone)]
pub struct ResourceStr<'a> {
  parts: &'a [ResourceStrPart<'a>],
  depth: usize,
}
impl<'a> ResourceStr<'a> {
  pub fn is_trimmed(self) -> bool {
    self.depth != 0
  }
  pub fn depth(self) -> usize {
    self.depth
  }
  pub fn full_str(self) -> ResourceStr<'a> {
    ResourceStr {
      parts: self.parts,
      depth: 0,
    }
  }
  pub fn iter(self) -> impl Iterator<Item = &'a ResourceStrPart<'a>> {
    self.parts.iter().skip(self.depth)
  }
  pub fn descend(self, levels: usize) -> ResourceStr<'a> {
    ResourceStr {
      parts: self.parts,
      depth: self.depth + levels,
    }
  }
  pub fn ascend(self, levels: usize) -> ResourceStr<'a> {
    ResourceStr {
      parts: self.parts,
      depth: self.depth - levels,
    }
  }
  pub fn remainder(self) -> Option<usize> {
    self.max_depth().checked_sub(self.depth)
  }
  pub fn max_depth(self) -> usize {
    self.parts.len()
  }
}
impl<'a> From<ResourceString<'a>> for ResourceStr<'a> {
  fn from(str: ResourceString<'a>) -> Self {
    str.build()
  }
}
impl<'a> Display for ResourceStr<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    for (i, part) in self.parts.iter().enumerate() {
      let sep = if i == self.depth { '>' } else { '.' };
      f.write_char(sep)?;
      f.write_str(part.as_str())?;
    }
    Ok(())
  }
}
