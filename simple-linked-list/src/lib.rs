type OptionalNode<T> = Option<Box<Node<T>>>;

struct Node<T> {
    data: T,
    tail: OptionalNode<T>
}

impl<T> Node<T> {
    pub fn new(data: T, tail: OptionalNode<T>) -> Self {
        Node {
            data,
            tail
        }
    }
}

pub struct SimpleLinkedList<T> {
    head: OptionalNode<T>
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList {
            head: None
        }
    }

    pub fn len(&self) -> usize {
        // should probabyl be implemented as a simple counter that 
        // decreases/increases in push/pop, but this is a more
        // interesting solution for sake of exploring Rust
        match self.head {
            Some(ref head) => {
                let mut count = 1;
                let mut n = &head.tail;
                while n.is_some() {
                    count += 1;
                    n = match n {
                        Some(t) => &t.tail,
                        None => &None
                    }
                }
                count
            },
            None => 0
        }
    }

    pub fn push(&mut self, element: T) {
        self.head = Some(Box::new(Node::new(element, self.head.take())));
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            let node = *node;
            self.head = node.tail;
            node.data
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            &node.data
        })
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| {
            &mut node.data
        })
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.as_ref().map(|n| &**n)
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            next: self.head.as_mut().map(|n| &mut **n)
        }
    }
}

impl<T: Clone> SimpleLinkedList<T> {
    pub fn rev(&self) -> SimpleLinkedList<T> {
        let mut list = SimpleLinkedList::new();
        let mut tail = &self.head;
        while let Some(ref node) = tail {
            list.push(node.data.clone());
            tail = &node.tail;
        }
        list
    }
}

impl<'a, T: Clone> From<&'a [T]> for SimpleLinkedList<T> {
    fn from(items: &[T]) -> Self {
        let mut list = SimpleLinkedList::new();
        items.iter().for_each(|item| {
            list.push(item.clone());
        });
        list
    }
}

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(mut self) -> Vec<T> {
        let mut vector = vec![];
        let mut tail = self.head.take();
        while let Some(node) = tail {
            let node = *node;
            tail = node.tail;
            vector.push(node.data);
        }
        vector.reverse();
        vector
    }
}

impl<T> Drop for SimpleLinkedList<T> {
    fn drop(&mut self) {
        let mut cur = self.head.take();
        while let Some(mut boxed_node) = cur {
            cur = boxed_node.tail.take();
        }
    }
}

pub struct IntoIter<T>(SimpleLinkedList<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop() 
    }
}

pub struct Iter<'a, T: 'a> {
    next: Option<&'a Node<T>>
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.tail.as_ref().map(|n| &**n);
            &node.data
        })
    }
}

pub struct IterMut<'a, T: 'a> {
    next: Option<&'a mut Node<T>>
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.tail.as_mut().map(|n| &mut **n);
            &mut node.data
        })
    }
}
