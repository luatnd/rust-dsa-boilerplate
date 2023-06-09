use std::fmt::Debug;
use std::ops::Div;
use std::time::Instant;

#[derive(Debug)]
pub struct TestCase<Input, Output>
  where Input: Debug,
        Input: Clone,
        Output: Debug,
        Output: PartialEq,
        Output: Eq,
{
  pub input: Input,
  pub output: Output,
}

// trait TestCase {
//   fn hash(&self) -> u64;
// }

pub fn manual_test<Input, Output>(
  id: &str,
  cases: Vec<TestCase<Input, Output>>,
  get_result: fn(Input) -> Output,
) where Input: Debug,
        Input: Clone,
        Output: Debug,
        Output: PartialEq,
        Output: Eq,
{
  for case in cases {
    println!("\n[{}_manual] case: {:?}", id, case.input);
    let actual = get_result(case.input);
    assert_eq!(actual, case.output, "Failed: actual={:?} expect={:?}", actual, case.output);
    println!("[{}_manual] OK: actual = expected = {:?}", id, case.output);
  }
}

pub fn manual_test_full<Input, Output>(
  id: &str,
  cases: Vec<TestCase<Input, Output>>,
  get_result: fn(Input, Output) -> Output,
) where Input: Debug,
        Input: Clone,
        Output: Debug,
        Output: PartialEq,
        Output: Eq,
        Output: Clone,
{
  for case in cases {
    println!("\n[{}_manual] case: {:?}", id, case.input);
    let actual = get_result(case.input, case.output.clone());
    assert_eq!(actual, case.output, "Failed: actual={:?} expect={:?}", actual, case.output);
    println!("[{}_manual] OK: actual = expected = {:?}", id, case.output);
  }
}

//
// TODO: Change to #[bench] if rust test is stable
// Because of its unstable so we need to benchmark ourself
//
pub fn bench_test<Input, Output>(
  id: &str,
  bench_loop: u32,
  heavy_input: Input,
  get_result: fn(Input) -> Output,
)
  where Input: Debug,
        Input: Clone,
        Output: Debug,
        Output: PartialEq,
        Output: Eq,
{
  let t1 = Instant::now();
  for _ in 0..bench_loop {
    let i = heavy_input.clone();
    get_result(i);
  }
  let t2 = Instant::now();
  let t = t2 - t1;
  println!("{}_bench: iter={:?} total={:?} avg={:?}", id, bench_loop, t, t.div(bench_loop));
}

pub fn stress_test<Input, Output>(
  id: &str,
  bench_loop: u32,
  generate_test_case: fn() -> Input,
  get_result: fn(Input) -> Output,
  get_trusted_result: fn(Input) -> Output,
)  where Input: Debug,
         Input: Clone,
         Output: Debug,
         Output: PartialEq,
         Output: Eq,
{
  for _ in 0..bench_loop {
    let input = generate_test_case();
    println!("\n[{}_stress] case: {:?}", id, input);

    let expected: Output = get_trusted_result(input.clone());
    let actual = get_result(input.clone());

    assert_eq!(actual, expected, "Failed: actual={:?} expect={:?}", actual, expected);
    println!("[{}_stress] OK", id);
  }
}
