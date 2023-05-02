use std::rc::Rc;
use std::cell::{RefCell};

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

// helper function for test
pub fn to_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
    let mut current = None;
    for &v in vec.iter().rev() {
        let mut node = ListNode::new(v);
        node.next = current;
        current = Some(Box::new(node));
    }
    current
}



pub type ListNode2Ref = Option<Rc<RefCell<ListNode2>>>;
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ListNode2 {
    pub val: i32,
    pub next: Option<Rc<RefCell<ListNode2>>>,
}
impl ListNode2 {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode2 { next: None, val }
    }
    pub fn from(vec: Vec<i32>) -> ListNode2Ref {
        let mut current = None;
        for &v in vec.iter().rev() {
            let mut node = ListNode2::new(v);
            node.next = current;
            current = Some(Rc::new(RefCell::new(node)));
        }
        current
    }
}



pub type ListNode3Ref = Option<Rc<Box<ListNode3>>>;
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ListNode3 {
    pub val: i32,
    pub next: ListNode3Ref,
}
impl ListNode3 {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode3 { next: None, val }
    }
    pub fn from(vec: Vec<i32>) -> ListNode3Ref {
        let mut current = None;
        for &v in vec.iter().rev() {
            let mut node = ListNode3::new(v);
            node.next = current;
            current = Some(Rc::new(Box::new(node)));
        }
        current
    }
}
fn traverse_linked_list(head: &ListNode3Ref) {
    let mut current_node = head;
    while let Some(node) = current_node {
        println!("{}", node.val);
        current_node = &node.next;
    }
}

#[macro_export]
macro_rules! linked {
    ($($e:expr),*) => {to_list(vec![$($e.to_owned()), *])};
    ($($e:expr,)*) => {to_list(vec![$($e.to_owned()), *])};
}
