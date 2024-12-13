use std::cmp::Ordering::{Equal, Less, Greater};

pub fn binary_search<T: Ord>(arr: &[T], target: T) -> Option<usize> {
  let mut left = 0;
  let mut right = arr.len();

  while left < right {
    let middle = left + (right - left) / 2;

    match arr[middle].cmp(&target) {
      Equal => return Some(middle),
      Less => left = middle + 1,
      Greater => right = middle
    }
  }

  None
}

#[cfg(test)]
mod tests {
  use super::*;

  test_search!(binary_search);
}
