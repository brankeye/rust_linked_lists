use std::mem;

pub struct List {
    head: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, value: i32) {
        let next = mem::replace(&mut self.head, Link::Empty);
        let node = Node {
            value,
            next,
        };
        self.head = Link::More(Box::new(node));
    }

    pub fn pop(&mut self) -> Option<i32> {
        let head = mem::replace(&mut self.head, Link::Empty);
        match head {
            Link::Empty => None,
            Link::More(node) => {
                let node = *node;
                self.head = node.next;
                Some(node.value)
            }
        }
    }
}

enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    value: i32,
    next: Link,
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None); 
    }
}
