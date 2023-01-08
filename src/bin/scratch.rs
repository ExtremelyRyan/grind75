use std::cell::{RefCell, RefMut};
use std::collections::HashMap;
use std::rc::Rc;

fn _refcell_example() {
    let shared_map: Rc<RefCell<_>> = Rc::new(RefCell::new(HashMap::new()));
    // Create a new block to limit the scope of the dynamic borrow
    {
        let mut map: RefMut<_> = shared_map.borrow_mut();
        map.insert("africa", 92388);
        map.insert("kyoto", 11837);
        map.insert("piccadilly", 11826);
        map.insert("marbles", 38);
    }
    // Note that if we had not let the previous borrow of the cache fall out
    // of scope then the subsequent borrow would cause a dynamic thread panic.
    // This is the major hazard of using `RefCell`.
    let total: i32 = shared_map.borrow().values().sum();
    println!("{total}");
}

fn main() {
    // refcell_example();
    tree_example();
}

fn tree_example() {
    let mut x = Node {
        val: "m",
        l: None,
        r: None,
    };
    x.insert("z");
    x.insert("b");
    x.insert("c");
    assert!(
        x == Node {
            val: "m",
            l: Some(Box::new(Node {
                val: "b",
                l: None,
                r: Some(Box::new(Node {
                    val: "c",
                    l: None,
                    r: None
                })),
            })),
            r: Some(Box::new(Node {
                val: "z",
                l: None,
                r: None
            })),
        }
    );
}

#[derive(PartialEq)]
struct Node<'a> {
    val: &'a str,
    l: Option<Box<Node<'a>>>,
    r: Option<Box<Node<'a>>>,
}
impl<'a> Node<'a> {
    pub fn insert(&mut self, new_val: &'a str) {
        if self.val == new_val {
            return;
        }
        let target_node = if new_val < self.val {
            &mut self.l
        } else {
            &mut self.r
        };
        match target_node {
            &mut Some(ref mut subnode) => subnode.insert(new_val),
            &mut None => {
                let new_node = Node {
                    val: new_val,
                    l: None,
                    r: None,
                };
                let boxed_node = Some(Box::new(new_node));
                *target_node = boxed_node;
            }
        }
    }
}
