use rand::Rng;

pub fn rand_char() -> char {
  let c: char = rand::thread_rng().gen_range('a'..='z');
  c
}

pub fn rand_string(n: usize) -> String {
  let mut s = "".to_string();
  for _ in 0..n {
    s += &rand_char().to_string();
  }
  s
}
