#[cfg(test)]
mod tests {
  use std::cell::RefCell;
  use std::ops::Add;
  use std::rc::Rc;
  use super::*;

  #[test]
  fn main() {
    let num = Rc::new(RefCell::new(5));

    // get
    let value_rc = Rc::clone(&num);
    let value_rc2 = Rc::clone(&num);

    let value = value_rc.borrow();
    println!("{:?}", value_rc); // out: RefCell { value: 5 }
    println!("{:?}", value); // out: 5


    // set:
    *value_rc2.borrow_mut() += 6; // panic
    println!("{}", value_rc2.borrow()); // out: 6
  }

  #[test]
  fn main_will_panic() {
    let num = Rc::new(RefCell::new(5));

    // get
    let value = num.borrow();
    println!("{}", value); // out: 5

    // set: PANIC: already borrowed: BorrowMutError
    let v2 = num.borrow_mut().add(1); // panic
    println!("{} {}", value, v2);
  }
}
