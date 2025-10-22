impl Solution {
  pub fn final_value_after_operations(operations: Vec<string>) -> i32 { 
    operations
      .into_iter()
      .map(|s| match s.as_bytes()[1] == b'+' {
        true => 1,
        false => -1,
      })
      .sum()
  }
}