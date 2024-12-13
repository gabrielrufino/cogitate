pub fn linear_search<T: PartialEq>(arr: &[T], target: T) -> Option<usize> {
  for (index, item) in arr.iter().enumerate() {
    if *item == target {
      return Some(index);
    }
  }

  None
}

#[cfg(test)]
mod tests {
  use super::*;

  test_search!(linear_search);
}
