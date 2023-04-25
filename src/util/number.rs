use rand::Rng;

pub fn rand_number(min: i32, max: i32) -> i32 {
  let num = rand::thread_rng().gen_range(min..=max);
  num.into()
}
