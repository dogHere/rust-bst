type Child<T> = Option<Box<Node<T>>>;

#[derive(Debug, Eq, PartialOrd, PartialEq)]
pub struct Node<T> {
    value: T,
    left: Child<T>,
    right: Child<T>,
}

impl<T: PartialOrd> Node<T> {
    fn new(value: T) -> Node<T> {
        Node {
            value,
            left: None,
            right: None,
        }
    }


    fn add(mut node: Node<T>, value: T) -> Node<T> {
        if value > node.value {
            if node.right.is_none() {
                node.right = Some(Box::from(Node::new(value)));
                node
            } else {
                node.right = Some(Box::from(Node::add(*node.right.unwrap(), value)));
                node
            }
        } else if value < node.value {
            if node.left.is_none() {
                node.left = Some(Box::from(Node::new(value)));
                node
            } else {
                node.left = Some(Box::from(Node::add(*node.left.unwrap(), value)));
                node
            }
        } else {
            node
        }
    }
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct Tree<T> {
    root: Child<T>
}

impl<T: PartialOrd> Tree<T> {
    pub fn new() -> Tree<T> {
        Tree {
            root: None
        }
    }

    pub fn push(mut self, value: T)->Tree<T> {
        if self.root.is_none() {
            self.root = Some(Box::from(Node::new(value)));
            self
        } else {
            let r = self.root.take();
            self.root = Some(Box::from(Node::add(*r.unwrap(), value)));
            self
        }
    }
}
