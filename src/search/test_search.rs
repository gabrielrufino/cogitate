#[macro_export]
macro_rules! test_search {
  ($search_fn: expr) => {
    // use std::time::Instant;

    #[test]
    fn test_element_found() {
      let numbers = vec![1, 3, 5, 7, 9];
      assert_eq!($search_fn(&numbers, 5), Some(2));
    }

    #[test]
    fn test_element_not_found() {
      let numbers = vec![1, 3, 5, 7, 9];
      assert_eq!($search_fn(&numbers, 4), None);
    }

    #[test]
    fn test_empty_array() {
      let numbers: Vec<i32> = vec![];
      assert_eq!($search_fn(&numbers, 1), None);
    }

    #[test]
    fn test_target_at_start() {
      let numbers = vec![10, 20, 30];
      assert_eq!($search_fn(&numbers, 10), Some(0));
    }

    #[test]
    fn test_target_at_end() {
      let numbers = vec![10, 20, 30];
      assert_eq!($search_fn(&numbers, 30), Some(2));
    }

    #[test]
    fn test_single_element_found() {
      let numbers = vec![42];
      assert_eq!($search_fn(&numbers, 42), Some(0));
    }

    #[test]
    fn test_single_element_not_found() {
      let numbers = vec![42];
      assert_eq!($search_fn(&numbers, 24), None);
    }

    /*
    #[test]
    fn test_performance() {
      let size: u64 = 1_000_000_000;
      let numbers: Vec<u64> = (1..=size).collect();
      let target = size;
    
      let start = Instant::now();
      let result = $search_fn(&numbers, target);
      let duration = start.elapsed();
      println!("Search: {:?}, Duration: {:?}", result, duration);
    }
    */
  };
}
