pub fn rand<T>(
  n: u32,
  rand_fn: fn() -> T,
) -> Vec<T> {
  rand_array(n, rand_fn)
}

pub fn rand_array<T>(
  n: u32,
  rand_fn: fn() -> T,
) -> Vec<T> {
  let vec: Vec<T> = (0..n)
    .map(|_| rand_fn())
    .collect();
  vec
}
