pub struct SelectionSort<'a, T: Ord> {
  arr: &'a mut [T],
}

impl<'a, T: Ord> SelectionSort<'a, T> {
  pub fn new(arr: &mut [T]) -> SelectionSort<T> {
    SelectionSort { arr }
  }

  pub fn execute(&mut self) {
    let length = self.arr.len();

    for i in 0..length {
      let mut min_index = i;

      for j in (i + 1)..length {
        if self.arr[j] < self.arr[min_index] {
          min_index = j;
        }
      }

      if min_index != i {
        self.arr.swap(i, min_index);
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  test_sort!(SelectionSort);
}
