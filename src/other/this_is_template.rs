///
/// Sum 2 number
///
/// NOTE: This is the boilerplate file used for all the problem
/// Support these kind tests: manual, stress, bench
///
pub struct Solution {}

impl Solution {
  pub fn sum(a: i32, b: i32) -> i32 {
    a + b
  }

  pub fn sum_naive(a: i32, b: i32) -> i32 {
    a * 2 + b - a
  }
}


#[cfg(test)]
pub mod tests {
  use super::*;
  use crate::util::{number};
  use crate::util::testing::{bench_test, manual_test, stress_test, TestCase};

  type Input = (i32, i32);
  type Output = i32;
  type Test = TestCase<Input, Output>;

  fn get_result(input: Input) -> Output {
    Solution::sum(input.0, input.1)
  }

  fn get_trusted_result(input: Input) -> Output {
    Solution::sum_naive(input.0, input.1)
  }

  #[test]
  fn id00xx_manual() {
    let cases = vec![
      Test {
        input: (1, 2),
        output: 3,
      },
      Test {
        input: (0, 3),
        output: 3,
      },
    ];

    manual_test("id00xx", cases, get_result);
  }


  #[test]
  fn id00xx_bench() {
    // return;

    let bench_loop = 1000;
    let heavy_input: Input = (
      number::rand_number(600, 1000),
      number::rand_number(900, 1000),
    );
    bench_test("id00xx", bench_loop, heavy_input, get_result);
  }

  #[test]
  fn id00xx_stress() {
    // return;

    let count = 1000;
    let generate_test_case = || -> Input {
      (
        number::rand_number(0, 100),
        number::rand_number(0, 100),
      )
    };

    stress_test("id00xx", count, generate_test_case, get_result, get_trusted_result);
  }
}
