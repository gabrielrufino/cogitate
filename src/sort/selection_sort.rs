pub fn selection_sort<T: Ord>(arr: &mut [T]) {
  let length = arr.len();

  for i in 0..length {
    let mut min_index = i;

    for j in (i + 1)..length {
      if arr[j] < arr[min_index] {
        min_index = j;
      }
    }

    if min_index != i {
      arr.swap(i, min_index);
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_empty_array() {
    let mut arr: Vec<i32> = vec![];
    selection_sort(&mut arr);
    assert_eq!(arr, vec![]);
  }

  #[test]
  fn test_single_element() {
    let mut arr = vec![1];
    selection_sort(&mut arr);
    assert_eq!(arr, vec![1]);
  }

  #[test]
  fn test_sorted_array() {
    let mut arr = vec![1, 2, 3, 4, 5];
    selection_sort(&mut arr);
    assert_eq!(arr, vec![1, 2, 3, 4, 5]);
  }

  #[test]
  fn test_reverse_sorted_array() {
    let mut arr = vec![5, 4, 3, 2, 1];
    selection_sort(&mut arr);
    assert_eq!(arr, vec![1, 2, 3, 4, 5]);
  }

  #[test]
  fn test_unsorted_array() {
    let mut arr = vec![64, 25, 12, 22, 11];
    selection_sort(&mut arr);
    assert_eq!(arr, vec![11, 12, 22, 25, 64]);
  }

  #[test]
  fn test_duplicates() {
    let mut arr = vec![3, 3, 2, 1, 1];
    selection_sort(&mut arr);
    assert_eq!(arr, vec![1, 1, 2, 3, 3]);
  }
}
