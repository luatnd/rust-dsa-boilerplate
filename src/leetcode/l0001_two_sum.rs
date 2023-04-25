/**
 * [1] Two Sum
 *
 * Given an array of integers, return indices of the two numbers such that they
 * add up to a specific target.
 *
 * You may assume that each input would have exactly one solution, and you may
 * not use the same element twice.
 *
 * Example:
 *
 *
 * Given nums = [2, 7, 11, 15], target = 9,
 *
 * Because nums[0] + nums[1] = 2 + 7 = 9,
 * return [0, 1].
 *
 */
use std::collections::HashMap;

pub struct Solution {}

impl Solution {
  pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::with_capacity(nums.len());
    for (index, num) in nums.iter().enumerate() {
      match map.get(&(target - num)) {
        None => {
          map.insert(num, index);
        }
        Some(sub_index) => return vec![*sub_index as i32, index as i32],
      }
    }
    vec![]
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  struct Test {
    input: (Vec<i32>, i32),
    output: Vec<i32>,
  }

  #[test]
  fn l0001_manual() {
    let cases = vec![
      Test {
        input: (vec![2, 7, 11, 15], 9),
        output: vec![0, 1],
      },
      Test {
        input: (vec![3, 2, 4], 6),
        output: vec![1, 2],
      },
    ];

    for case in cases {
      assert_eq!(
        Solution::two_sum(case.input.0, case.input.1),
        case.output
      )
    }
  }

  #[test]
  fn l0001_stress() {}

  #[test]
  fn l0001_bench() {}
}
