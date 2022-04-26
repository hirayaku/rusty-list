/// A single linked list with push, pop, insert, split operations

// use Option<Box> as a nullable pointer in place of Enum in first.rs

pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

pub struct Node<T> {
    elem: T,
    link: Link<T>,
}

pub struct IntoIter<T>(List<T>);
// To not move list, need 'a, which is a lifetime hint to compiler?
// pub struct IntoIter<'a, T>(&'a mut List<T>);
impl <T> List<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}
impl <T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

/*
pub struct Iter<T> {
    next: Option<&Node<T>>,
}
impl <T> List<T> {
    pub fn iter(&self) -> Iter<T> {
        Iter { next: self.head.map(|node| &node) }
    }
}
impl <T> Iterator for Iter<T> {
    type Item = &T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|
    }
}
*/

impl <T> List<T> {
    /// create an empty list
    pub fn new() -> Self {
        List {head: None}
    }
    pub fn empty(&self) -> bool {
        self.head.is_none()
    }
    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|boxed_node_ref| {
            &boxed_node_ref.elem
        })
    }
    /// return the head node
    pub fn head(&mut self) -> &mut Node<T> {
        match &mut self.head {
            None => panic!("List has no head!"),
            Some(boxed_node) => &mut*boxed_node
        }
    }
    /// push a new element as the head node
    pub fn push(&mut self, elem: T) {
        self.head = Some(Box::new(
            Node {elem: elem, link: self.head.take()})
        );
    }
    /// pop the element in the head node
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|boxed_node| {
            self.head = boxed_node.link;
            boxed_node.elem
        })
    }
}

impl <T> Drop for List<T> {
    /// iterative drop to prevent stack overflow
    fn drop(&mut self) {
        // transfer ownership of the current Box<Node> from list to `boxed_node`
        while let Some(boxed_node) = self.head.take() {
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
        let boxed_node = Box::new(Node {
            elem: elem, link: self.link.take()
        });
        self.link = Some(boxed_node);
    }
    /// split the list at the current node
    /// return the new list whose head is the next node
    pub fn split(&mut self) -> List<T> {
        List { head: self.link.take() }
    }
    pub fn has_next(&self) -> bool {
        self.link.is_some()
    }
    pub fn next(&mut self) -> &mut Node<T> {
        match &mut self.link {
            None => panic!("Node has no next!"),
            Some(boxed_node) => &mut*boxed_node,
        }
    }
}

