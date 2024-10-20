mod merge;
pub use merge::MergeIter;
pub mod serde_array;
mod sorted;
pub use sorted::IsSorted;
mod end_sort;
pub use end_sort::EndSort;

pub fn from_alpha(c: char) -> u8 {
  debug_assert!(
    c.is_ascii_lowercase(),
    "Character '{}' out of supported range",
    c
  );
  c as u8 - b'a'
}

pub fn from_word(s: String) -> Vec<u8> {
  s.chars().map(from_alpha).collect()
}

pub fn into_alpha(v: u8) -> char {
  debug_assert!((0..26).contains(&v), "Value '{}' out of supported range", v);
  (v + b'a') as char
}

pub fn into_word(v: Vec<u8>) -> String {
  v.into_iter().map(into_alpha).collect()
}
