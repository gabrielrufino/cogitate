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

  #[test]
  fn test_element_found() {
    let numbers = vec![1, 3, 5, 7, 9];
    let target = 5;
    assert_eq!(linear_search(&numbers, target), Some(2));
  }

  #[test]
  fn test_element_not_found() {
    let numbers = vec![1, 3, 5, 7, 9];
    let target = 4;
    assert_eq!(linear_search(&numbers, target), None);
  }

  #[test]
  fn test_empty_array() {
    let numbers: Vec<i32> = vec![];
    let target = 1;
    assert_eq!(linear_search(&numbers, target), None);
  }

  #[test]
  fn test_multiple_occurrences() {
    let numbers = vec![1, 3, 5, 5, 9];
    let target = 5;
    assert_eq!(linear_search(&numbers, target), Some(2));
  }

  #[test]
  fn test_strings() {
    let words = vec!["apple", "banana", "cherry"];
    let target = "banana";
    assert_eq!(linear_search(&words, target), Some(1));
  }

  #[test]
  fn test_target_at_start() {
    let numbers = vec![10, 20, 30];
    let target = 10;
    assert_eq!(linear_search(&numbers, target), Some(0));
  }

  #[test]
  fn test_target_at_end() {
    let numbers = vec![10, 20, 30];
    let target = 30;
    assert_eq!(linear_search(&numbers, target), Some(2));
  }
}
