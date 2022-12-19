// Rust-101, Part 16: Unsafe Rust, Drop
// ====================================

use std::{fmt::Debug, ptr};

// A node of the list consists of the data, and two node pointers for the predecessor and successor.
struct Node<T: Clone + Debug> {
    next: NodePtr<T>,
    prev: NodePtr<T>,
    data: T,
}
// A node pointer is a *mutable raw pointer* to a node.
type NodePtr<T> = *mut Node<T>;

// The linked list itself stores pointers to the first and the last node. In addition, we tell Rust
// that this type will own data of type `T`.
pub struct LinkedList<T: Clone + Debug> {
    head: NodePtr<T>,
    tail: NodePtr<T>,
}

impl<T: Clone + Debug> LinkedList<T> {
    // A new linked list just contains null pointers. `PhantomData` is how we construct any
    // `PhantomData<T>`.
    pub fn new() -> Self {
        LinkedList {
            head: ptr::null_mut(),
            tail: ptr::null_mut(),
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.tail.is_null() {
            return None;
        };

        unsafe {
            let ptr = Box::from_raw(self.tail);
            if self.head == self.tail {
                self.head = std::ptr::null_mut();
                self.tail = std::ptr::null_mut();
            } else {
                self.tail = (*self.tail).prev;
                (*self.tail).next = std::ptr::null_mut();
            }

            return Some(ptr.data.clone());
        };
    }
    pub fn clear(&mut self) {
        let mut cur_ptr = self.head;
        while !cur_ptr.is_null() {
            unsafe {
                let cur = Box::from_raw(cur_ptr);

                cur_ptr = cur.next;
            }
        }
        self.head = std::ptr::null_mut();
        self.tail = std::ptr::null_mut();
    }

    // This function adds a new node to the end of the list.
    pub fn push(&mut self, t: T) {
        // Create the new node, and make it a raw pointer.
        let new = Box::into_raw(Box::new(Node {
            data: t,
            next: ptr::null_mut(),
            prev: ptr::null_mut(),
        }));

        // Update other pointers to this node.
        if self.tail.is_null() {
            // The list is currently empty, so we have to update the head pointer.
            self.head = new;
            self.tail = new;
        } else {
            // We have to update the `next` pointer of the tail node.
            unsafe {
                (*self.tail).next = new;
                (*new).prev = self.tail;
                self.tail = new;
            }
        }
        // Make this the last node.
    }

    // **Exercise 16.1**: Add some more operations to `LinkedList`: `pop_back`, `push_front` and
    // `pop_front`. Add testcases for `push_back` and all of your functions. The `pop` functions
    // should take `&mut self` and return `Option<T>`.
}

// **Exercise 16.2**: Add a method `iter` and a type `Iter` providing iteration for shared
// references. Add testcases for both kinds of iterators.

// ## `Drop`

impl<T> Drop for Node<T>
where
    T: Clone + Debug,
{
    fn drop(&mut self) {
        println!("{:?} dropped", self.data);
    }
}

impl<T: Clone + Debug> Drop for LinkedList<T> {
    // The destructor itself is a method which takes `self` in mutably borrowed form. It cannot own
    // `self`, because then the destructor of `self` would be called at the end of the function,
    // resulting in endless recursion.
    fn drop(&mut self) {
        let mut cur_ptr = self.head;
        while !cur_ptr.is_null() {
            unsafe {
                let cur = Box::from_raw(cur_ptr);

                cur_ptr = cur.next;
            }
        }
        println!("linkedlist dropped");
    }
}

fn main() {
    let mut l = LinkedList::new();
    l.push(5);
    l.push(10);
    let a = l.pop();
    let b = l.pop();
    // println!("{} ", a.unwrap());
    // println!("{} ", b.unwrap());
}

// ## The End
