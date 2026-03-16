pub struct BubbleSort<'a, T: PartialOrd> {
  arr: &'a mut [T],
}

impl<'a, T: PartialOrd> BubbleSort<'a, T> {
  pub fn new(arr: &'a mut [T]) -> Self {
    BubbleSort { arr }
  }

  pub fn execute(&mut self) {
    let length = self.arr.len();

    for i in 0..length {
      let mut swapped = false;

      for j in 0..length - i - 1 {
        if self.arr[j] > self.arr[j + 1] {
          self.arr.swap(j, j + 1);
          swapped = true
        }
      }

      if !swapped {
        break;
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  test_sort!(BubbleSort);
}
