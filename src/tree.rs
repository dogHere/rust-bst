//! bst detail.
use std::mem::swap;

type Child<T> = Option<Box<Node<T>>>;

#[derive(Debug, Eq, PartialOrd, PartialEq, Clone)]
pub struct Node<T> {
    pub value: T,
    pub left: Child<T>,
    pub right: Child<T>,
}

impl<T: PartialOrd + Clone> Node<T> {
    fn new(value: T) -> Node<T> {
        Node {
            value,
            left: None,
            right: None,
        }
    }


    fn value(&self) -> T {
        self.value.clone()
    }

    fn push(&mut self, value: T) {
        if value > self.value {
            match self.right {
                None => {
                    swap(&mut self.right, &mut Some(Box::from(Node::new(value))));
                }
                Some(ref mut n) => {
                    n.push(value)
                }
            }
        } else if value < self.value {
            match self.left {
                None => {
                    swap(&mut self.left, &mut Some(Box::from(Node::new(value))));
                }
                Some(ref mut n) => {
                    n.push(value)
                }
            }
        }
    }

    fn min(&self) -> T {
        match self.left {
            None => self.value.clone(),
            Some(ref n) => Node::min(&*n)
        }
    }

    fn max(&self) -> T {
        match self.right {
            None => self.value.clone(),
            Some(ref n) => Node::max(&*n)
        }
    }

    fn find(&self, value: &T) -> Option<Box<&Node<T>>> {
        if value > &self.value {
            match self.right {
                None => None,
                Some(ref n) => Node::find(&*n, &value)
            }
        } else if value < &self.value {
            match self.left {
                None => None,
                Some(ref n) => Node::find(&*n, &value)
            }
        } else {
            Some(Box::from(self))
        }
    }


    fn delete(node: &mut Child<T>, value: &T) {
        let mut all_none = false;
        match *node {
            None => {}
            Some(ref mut r) => {
                if !r.left.is_some() && !r.right.is_some() {
                    all_none = true;
                } else {
                    if value > &r.value() {
                        Node::delete(&mut r.right, value);
                    } else if value < &r.value() {
                        Node::delete(&mut r.left, value);
                    } else {
                        if r.right.is_none() {
                            let t = r.left.take();
                            swap(&mut r.value, &mut t.unwrap().value);
                            swap(&mut None, &mut r.left);
                        } else if r.left.is_none() {
                            let t = r.right.take();
                            swap(&mut r.value, &mut t.unwrap().value);
                            swap(&mut None, &mut r.right);
                        } else {
                            let mut m = r.right.take().unwrap().min();
                            swap(&mut None, &mut r.find(&m));
                            swap(&mut r.value, &mut m);
                        }
                    }
                }
            }
        }
        if all_none {
            swap(node, &mut None);
        }
    }
}

#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub struct Tree<T> {
    root: Child<T>
}

impl<T: PartialOrd + Clone> Tree<T> {


    /// Create new instance of Tree.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate bst_rs;
    /// # use bst_rs::tree::*;
    /// #
    /// # fn main(){
    ///     let  tree:Tree<i32> = Tree::new();
    /// # }
    /// ```
    pub fn new() -> Tree<T> {
        Tree {
            root: None
        }
    }

    /// Push `value` into a tree.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate bst_rs;
    /// # use bst_rs::tree::*;
    /// #
    /// # fn main(){
    ///     let mut tree = Tree::new();
    ///     tree.push("a")
    ///         .push("b")
    ///         .push("5")
    ///         .push("0")
    ///         .push("6")
    ///         .push("z");
    /// # }
    /// ```
    pub fn push(&mut self, value: T) -> Box<&mut Tree<T>> {
        match self.root {
            None => {
                swap(&mut self.root, &mut Some(Box::from(Node::new(value))));
            }
            Some(ref mut n) => {
                n.push(value);
            }
        }
        Box::new(self)
    }

    /// Get a copy of the min value from a tree.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate bst_rs;
    /// #
    /// # use bst_rs::tree::*;
    /// #
    /// # fn main(){
    ///     let mut tree = Tree::new();
    ///     tree.push("a")
    ///         .push("b")
    ///         .push("5")
    ///         .push("0")
    ///         .push("6")
    ///         .push("z");
    ///     assert_eq!(tree.min(),Some("0"));
    /// # }
    ///
    ///
    /// ```
    pub fn min(&self) -> Option<T> {
        match self.root {
            None => None,
            Some(ref n) => Some(n.min())
        }
    }

    /// Get a copy of the max value from a tree.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate bst_rs;
    /// #
    /// # use bst_rs::tree::*;
    /// #
    /// # fn main(){
    ///     let mut tree = Tree::new();
    ///     tree.push("a")
    ///         .push("b")
    ///         .push("5")
    ///         .push("0")
    ///         .push("6")
    ///         .push("z");
    ///     assert_eq!(tree.max(),Some("z"));
    /// # }
    /// ```
    pub fn max(&self) -> Option<T> {
        match self.root {
            None => None,
            Some(ref n) => Some(n.max())
        }
    }

    /// Get a ref of the `value` in a tree.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate bst_rs;
    /// #
    /// # use bst_rs::tree::*;
    /// #
    /// # fn main(){
    ///     let mut tree = Tree::new();
    ///     tree.push("a")
    ///         .push("b")
    ///         .push("5")
    ///         .push("0")
    ///         .push("6")
    ///         .push("z");
    ///
    ///     println!("{:?}",tree.find(&"5"));
    ///     assert_eq!(tree.find(&"100"),None);
    /// # }
    /// ```
    pub fn find(&self, value: &T) -> Option<Box<&Node<T>>> {
        match self.root {
            None => None,
            Some(ref n) => n.find(value)
        }
    }

    /// Test the `value` if exists in a tree.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate bst_rs;
    /// #
    /// # use bst_rs::tree::*;
    /// #
    /// # fn main(){
    ///     let mut tree = Tree::new();
    ///     tree.push("a")
    ///         .push("b")
    ///         .push("5")
    ///         .push("0")
    ///         .push("6")
    ///         .push("z");
    ///
    ///     assert!(tree.exists(&"5"));
    ///
    ///     assert!(!tree.exists(&"100"));
    /// # }
    /// ```
    pub fn exists(&self, value: &T) -> bool {
        match self.find(value) {
            None => false,
            _ => true
        }
    }

    /// Delete `value` from a tree.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate bst_rs;
    /// #
    /// # use bst_rs::tree::*;
    /// #
    /// # fn main(){
    ///     let mut tree = Tree::new();
    ///     tree.push("a")
    ///         .push("b")
    ///         .push("5")
    ///         .push("0")
    ///         .push("6")
    ///         .push("z");
    ///
    ///     tree.delete(&"0");
    ///     println!("{:?}",tree);
    ///
    ///     tree.delete(&"10");
    ///     println!("{:?}",tree);
    ///
    ///     tree.delete(&"b");
    ///     println!("{:?}",tree);
    ///
    ///     tree.delete(&"a");
    ///     println!("{:?}",tree);
    /// # }
    /// ```
    pub fn delete(&mut self, value: &T) -> Box<&mut Tree<T>> {
        Node::delete(&mut self.root, value);
        Box::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn new_tree() {
        let tree: Tree<i32> = Tree::new();
        assert_eq!(tree, Tree {
            root: None
        })
    }

    fn create_tree() -> Tree<&'static str> {
        let mut tree = Tree::new();
        tree.push("a")
            .push("b")
            .push("5")
            .push("0")
            .push("6")
            .push("z");
        tree
    }

    #[test]
    fn tree_push() {
        let tree = create_tree();
        assert_eq!(tree, Tree {
            root: Some(Box::from(Node {
                value: "a",
                left: Some(Box::from(Node {
                    value: "5",
                    left: Some(Box::from(Node {
                        value: "0",
                        left: None,
                        right: None,
                    })),
                    right: Some(Box::from(Node {
                        value: "6",
                        left: None,
                        right: None,
                    })),
                })),
                right: Some(Box::from(Node {
                    value: "b",
                    left: None,
                    right: Some(Box::from(Node {
                        value: "z",
                        left: None,
                        right: None,
                    })),
                })),
            }))
        });
    }

    #[test]
    fn tree_max() {
        let tree = create_tree();
        assert_eq!(tree.max(), Some("z"));
    }

    #[test]
    fn tree_min() {
        let tree = create_tree();
        assert_eq!(tree.min(), Some("0"));
    }

    #[test]
    fn tree_find() {
        let tree = create_tree();
        assert_eq!(tree.find(&"5"), Some(Box::from(&Node {
            value: "5",
            left: Some(Box::from(Node {
                value: "0",
                left: None,
                right: None,
            })),
            right: Some(Box::from(Node {
                value: "6",
                left: None,
                right: None,
            })),
        })));

        assert_eq!(tree.find(&"100"), None);
    }

    #[test]
    fn tree_exists(){
        let tree = create_tree();

        assert!(tree.exists(&"5"));
        assert!(!tree.exists(&"100"));
    }

    #[test]
    fn tree_delete(){
        let mut tree = create_tree();
        tree.delete(&"0");

        assert_eq!(tree, Tree {
                root: Some(Box::from(Node {
                    value: "a",
                    left: Some(Box::from(Node {
                        value: "5",
                        left: None,
                        right: Some(Box::from(Node {
                            value: "6",
                            left: None,
                            right: None,
                        })),
                    })),
                    right: Some(Box::from(Node {
                        value: "b",
                        left: None,
                        right: Some(Box::from(Node {
                            value: "z",
                            left: None,
                            right: None,
                        })),
                    })),
                }))
            }
        );

        tree.delete(&"10");
        assert_eq!(tree, Tree {
            root: Some(Box::from(Node {
                value: "a",
                left: Some(Box::from(Node {
                    value: "5",
                    left: None,
                    right: Some(Box::from(Node {
                        value: "6",
                        left: None,
                        right: None,
                    })),
                })),
                right: Some(Box::from(Node {
                    value: "b",
                    left: None,
                    right: Some(Box::from(Node {
                        value: "z",
                        left: None,
                        right: None,
                    })),
                })),
            }))
        });

        tree.delete(&"b");
        assert_eq!(tree, Tree {
            root: Some(Box::from(Node {
                value: "a",
                left: Some(Box::from(Node {
                    value: "5",
                    left: None,
                    right: Some(Box::from(Node {
                        value: "6",
                        left: None,
                        right: None,
                    })),
                })),
                right: Some(Box::from(Node {
                    value: "z",
                    left: None,
                    right:None,
                })),
            }))
        });

        tree.delete(&"a");
        assert_eq!(tree, Tree {
            root: Some(Box::from(Node {
                value: "z",
                left: Some(Box::from(Node {
                    value: "5",
                    left: None,
                    right: Some(Box::from(Node {
                        value: "6",
                        left: None,
                        right: None,
                    })),
                })),
                right: None,
            }))
        });

    }
}
