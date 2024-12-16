pub fn bubble_sort<T: PartialOrd>(arr: &mut [T]) {
  let length = arr.len();

  for i in 0..length {
    let mut swapped = false;

    for j in 0..length - i - 1 {
      if arr[j] > arr[j + 1] {
        arr.swap(j, j + 1);
        swapped = true
      }
    }

    if !swapped {
      break;
    }
  }
}

mod tests {
  use super::*;

  test_sort!(bubble_sort);
}
