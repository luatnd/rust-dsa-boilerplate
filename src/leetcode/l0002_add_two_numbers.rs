// extern crate test;

/**
 * [2] Add Two Numbers
 *
 * You are given two non-empty linked lists representing two non-negative
 * integers. The digits are stored in reverse order and each of their nodes
 * contain a single digit. Add the two numbers and return it as a linked list.
 *
 * You may assume the two numbers do not contain any leading zero, except the
 * number 0 itself.
 *
 * Example:
 *
 *
 * Input: (2 -> 4 -> 3) + (5 -> 6 -> 4)
 * Output: 7 -> 0 -> 8
 * Explanation: 342 + 465 = 807.
 *
 */
use crate::util::linked_list::{ListNode, to_list};

pub struct Solution {}

impl Solution {
  pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
  ) -> Option<Box<ListNode>> {
    let (mut l1, mut l2) = (l1, l2);
    let mut dummy_head = Some(Box::new(ListNode::new(0)));
    let mut tail = &mut dummy_head;
    let (mut l1_end, mut l2_end, mut overflow) = (false, false, false);
    loop {
      let lhs = match l1 {
        Some(node) => {
          l1 = node.next;
          node.val
        }
        None => {
          l1_end = true;
          0
        }
      };
      let rhs = match l2 {
        Some(node) => {
          l2 = node.next;
          node.val
        }
        None => {
          l2_end = true;
          0
        }
      };
      // if l1, l2 end and there is not overflow from previous operation, return the result
      if l1_end && l2_end && !overflow {
        break dummy_head.unwrap().next;
      }
      let sum = lhs + rhs + if overflow { 1 } else { 0 };
      let sum = if sum >= 10 {
        overflow = true;
        sum - 10
      } else {
        overflow = false;
        sum
      };
      tail.as_mut().unwrap().next = Some(Box::new(ListNode::new(sum)));
      tail = &mut tail.as_mut().unwrap().next
    }
  }

  pub fn add_two_numbers_naive(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
  ) -> Option<Box<ListNode>> {
    to_list(vec![])
  }
}






#[cfg(test)]
pub mod tests {
  use super::*;
  // use test::{Bencher};

  use crate::util::{array, number};
  use crate::util::linked_list::{to_list};
  use crate::util::testing::{bench_test, manual_test, stress_test, TestCase};

  type Input = (Option<Box<ListNode>>, Option<Box<ListNode>>);
  type Output = Option<Box<ListNode>>;
  type Test = TestCase<Input, Output>;

  fn get_result(input: Input) -> Output {
    Solution::add_two_numbers(input.0, input.1)
  }

  fn get_trusted_result(input: Input) -> Output {
    Solution::add_two_numbers_naive(input.0, input.1)
  }

  #[test]
  fn l0002_manual() {
    let cases = vec![
      Test {
        input: (to_list(vec![2, 4, 3]), to_list(vec![5, 6, 4])),
        output: to_list(vec![7, 0, 8]),
      },
      Test {
        input: (to_list(vec![9, 9, 9, 9]), to_list(vec![9, 9, 9, 9, 9, 9])),
        output: to_list(vec![8, 9, 9, 9, 0, 0, 1]),
      },
      Test {
        input: (to_list(vec![0]), to_list(vec![0])),
        output: to_list(vec![0]),
      },
    ];

    manual_test("l0002", cases, get_result);
  }


  #[test]
  fn l0002_bench() {
    let bench_loop = 1000;
    let heavy_input: Input = (
      to_list(array::rand_array(100, || number::rand_number(0, 9))),
      to_list(array::rand_array(100, || number::rand_number(0, 9))),
    );
    bench_test("l0002", bench_loop, heavy_input, get_result);
  }

  #[test]
  fn l0002_stress() {
    let count = 1000;
    let generate_test_case = || -> Input {
      let n1 = number::rand_number(1, 100) as u32;
      let n2 = number::rand_number(1, 100) as u32;
      (
        to_list(array::rand_array(n1, || number::rand_number(0, 9))),
        to_list(array::rand_array(n2, || number::rand_number(0, 9))),
      )
    };

    stress_test("l0002", count, generate_test_case, get_result, get_trusted_result);
  }

  // #[bench]
  // fn l0002_bench_bench(b: &mut Bencher) {
  //   let heavy_input: Input = (
  //     to_list(util::array::rand_array(100, || util::number::rand_number(0, 9))),
  //     to_list(util::array::rand_array(100, || util::number::rand_number(0, 9))),
  //   );
  //   b.iter(|| get_result((heavy_input.0.clone(), heavy_input.1.clone())));
  // }
}


