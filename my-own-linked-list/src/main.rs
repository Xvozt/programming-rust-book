struct Node<T> {
    data: Box<T>,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(data: T) -> Self {
        Self {
            data: Box::new(data),
            next: None,
        }
    }

    fn next(&self) -> Option<&Self> {
        if let Some(next) = &self.next {
            Some(next.as_ref())
        } else {
            None
        }
    }

    fn mut_tail(&mut self) -> &mut Self {
        match self.next {
            Some(ref mut next) => next.mut_tail(),
            None => self,
        }
    }

    fn data(&self) -> &T {
        self.data.as_ref()
    }
}

struct SingleLinkedList<T> {
    head: Node<T>,
}

impl<T> SingleLinkedList<T> {
    fn new(data: T) -> Self {
        SingleLinkedList {
            head: Node::new(data),
        }
    }

    fn append(&mut self, data: T) {
        let tail = self.head.mut_tail();
        tail.next = Some(Box::new(Node::new(data)))
    }

    fn head(&self) -> &Node<T> {
        &self.head
    }
}

fn main() {
    let mut list = SingleLinkedList::new("head");
    list.append("middle");
    list.append("tail");
    let mut item = list.head();
    loop {
        println!("Item: {}", item.data());
        if let Some(next_item) = item.next() {
            item = next_item
        } else {
            break;
        }
    }
}
