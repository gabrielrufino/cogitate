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

  test_sort!(selection_sort);
}
