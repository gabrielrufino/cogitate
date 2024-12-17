pub fn fibonacci(n: u32) -> u64 {
  match n {
    0 => 0,
    1 => 1,
    _ => fibonacci(n - 1) + fibonacci(n - 2)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_fibonacci_base_cases() {
    assert_eq!(fibonacci(0), 0);
    assert_eq!(fibonacci(1), 1);
  }

  #[test]
  fn test_fibonacci_small_numbers() {
    assert_eq!(fibonacci(2), 1);
    assert_eq!(fibonacci(3), 2);
    assert_eq!(fibonacci(4), 3);
    assert_eq!(fibonacci(5), 5);
  }

  #[test]
  fn test_fibonacci_larger_numbers() {
    assert_eq!(fibonacci(10), 55);
    assert_eq!(fibonacci(15), 610);
  }
}
