/// A single linked list with push, pop, insert, split operations

use std::mem;

pub struct List<T> {
    head: Link<T>,
}

enum Link<T> {
    Empty,
    Ptr(Box<Node<T>>),
}

pub struct Node<T> {
    elem: T,
    link: Link<T>,
}

impl <T> List<T> {
    /// create an empty list
    pub fn new() -> Self {
        List {head: Link::Empty}
    }
    pub fn empty(&self) -> bool {
        match self.head {
            Link::Empty => true,
            Link::Ptr(_) => false
        }
    }
    /// return the head node
    pub fn head(&mut self) -> &mut Node<T> {
        match &mut self.head {
            Link::Empty => panic!("List has no head!"),
            Link::Ptr(box_node) => &mut*box_node
        }
    }
    /// push a new element as the head node
    pub fn push(&mut self, elem: T) {
        // NB: through a mut reference you could do virtually
        // everything except stealing a value and try owning it
        // the following code doesn't work
        //
        // self.head = Link::Ptr(Box::new(
        //     Node {elem: elem, link: self.head})
        // );

        self.head = Link::Ptr(Box::new(
            Node {elem: elem, link: mem::replace(&mut self.head, Link::Empty)})
        );
    }
    /// pop the element in the head node
    pub fn pop(&mut self) -> Option<T> {
        // NB: the return value should own the element,
        // so we have to forego ownership of head node
        // the following code doesn't work
        //
        // match &mut self.head {
        //     Link::Empty => None,
        //     Link::Ptr(box_node) => {
        //         self.head = box_node.link;
        //         Some(box_node.elem)
        //     }
        // }
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::Ptr(box_node) => {
                self.head = box_node.link;
                Some(box_node.elem)
            }
        }
    }
}

impl <T> Drop for List<T> {
    /// iterative drop to prevent stack overflow
    fn drop(&mut self) {
        // transfer ownership of the current Box<Node> from list to `boxed_node`
        while let Link::Ptr(boxed_node) = mem::replace(&mut self.head, Link::Empty) {
            self.head = boxed_node.link;
            // boxed_node then goes out scope and memory deallocated
        }
    }
}

impl <T> Node<T> {
    // TODO: insert and split is defined on a predecessor node
    // better to define them on an "itertor" type

    /// insert elem after the current node
    pub fn insert(&mut self, elem: T) {
        let box_node = Box::new(Node {
            elem: elem, link: mem::replace(&mut self.link, Link::Empty)
        });
        self.link = Link::Ptr(box_node);
    }
    /// split the list at the current node
    /// return the new list whose head is the next node
    pub fn split(&mut self) -> List<T> {
        List { head: mem::replace(&mut self.link, Link::Empty) }
    }
    pub fn has_next(&self) -> bool {
        match &self.link {
            Link::Empty => false,
            _ => true,
        }
    }
    pub fn next(&mut self) -> &mut Node<T> {
        match &mut self.link {
            Link::Empty => panic!("Node has no next!"),
            Link::Ptr(box_node) => &mut*box_node,
        }
    }
}

