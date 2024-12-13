pub fn binary_search<T: Ord>(arr: &[T], target: T) -> Option<usize> {
  let mut left = 0;
  let mut right = arr.len();

  while left < right {
    let middle = left + (right - left) / 2;

    if arr[middle] == target {
      return Some(middle)
    } else if arr[middle] < target {
      left = middle + 1
    } else {
      right = middle
    }
  }

  None
}

#[cfg(test)]
mod tests {
  use super::*;

  test_search!(binary_search);
}
