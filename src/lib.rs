pub mod first;
pub mod second;

#[cfg(test)]
mod tests {
    use super::second::List;

    #[test]
    fn syntax_check() {
        assert_eq!(None == Some(3), false);
    }

    #[test]
    fn push_pop_works() {
        let mut list: List<isize> = List::new();

        assert_eq!(list.pop(), None);

        list.push(3);
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), None);

        list.push(0);
        list.push(1);
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(0));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn peek_works() {
        let mut list: List<isize> = List::new();

        assert_eq!(list.peek(), None);

        list.push(3);
        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.peek(), None);
    }

    #[test]
    fn iter_works() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn insert_works() {
        let mut list: List<isize> = List::new();
        list.push(0);
        list.push(1);
        list.push(2);

        {
            let node = list.head();
            node.insert(3);
        }

        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(3));

        let mut node = list.head();
        // traverse to the tail node
        while node.has_next() {
            node = node.next();
        }
        node.insert(3);
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(0));
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn split_works() {
        let mut list: List<isize> = List::new();
        list.push(0);
        list.push(1);
        list.push(2);

        let mut new_list = list.head().split();
        assert_eq!(new_list.pop(), Some(1));
        assert_eq!(new_list.pop(), Some(0));

        let mut node = list.head();
        while node.has_next() {
            node = node.next();
        }
        assert_eq!(node.split().pop(), None);
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn drop_no_overflow() {
        let mut list: List<isize> = List::new();
        let mut i = 0;
        while i < 1000000 {
            // must have custom destructor defined to prevent stack overflow
            list.push(i);
            i += 1;
        }
    }
}
