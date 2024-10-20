use super::{ReadNode, ThinNode, WriteNode};
use crate::utils::serde_array;
use serde::{Deserialize, Serialize};
use std::array;

/// A full width node, capable of representing children<br>
/// that are potentially non-contiguous.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct WideNode<const N: usize> {
  #[serde(default, skip_serializing_if = "end_skip")]
  pub(crate) end: bool,
  #[serde(default = "children_default", skip_serializing_if = "children_skip")]
  #[serde(with = "serde_array")]
  pub(crate) children: [usize; N],
}

fn end_skip(end: &bool) -> bool {
  *end
}
fn children_skip<const N: usize>(children: &[usize; N]) -> bool {
  children == &[0; N]
}
fn children_default<const N: usize>() -> [usize; N] {
  [0; N]
}

impl<const N: usize> Default for WideNode<N> {
  fn default() -> Self {
    Self {
      end: false,
      children: [0; N],
    }
  }
}

impl<const N: usize> ReadNode for WideNode<N> {
  type Idx = usize;

  fn is_empty(&self) -> bool {
    self.children == [0; N]
  }

  fn is_end(&self) -> bool {
    self.end
  }

  fn has(&self, c: u8) -> bool {
    self.children[c as usize] > 0
  }

  fn get(&self, c: u8) -> Self::Idx {
    self.children[c as usize]
  }
}

impl<const N: usize> From<ThinNode> for WideNode<N> {
  fn from(value: ThinNode) -> Self {
    Self {
      end: value.is_end(),
      children: array::from_fn(|i| value.get(i as u8)),
    }
  }
}

impl<const N: usize> WriteNode for WideNode<N> {
  fn is_end_mut(&mut self) -> &mut bool {
    &mut self.end
  }

  fn get_mut(&mut self, c: u8) -> &mut Self::Idx {
    &mut self.children[c as usize]
  }
}
