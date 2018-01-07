type Child<T> = Option<Box<Node<T>>>;

#[derive(Debug, Eq, PartialOrd, PartialEq,Clone)]
pub struct Node<T> {
    value: T,
    left: Child<T>,
    right: Child<T>,
}

impl<T: PartialOrd+Clone> Node<T> {
    fn new(value: T) -> Node<T> {
        Node {
            value,
            left: None,
            right: None,
        }
    }


    fn push(mut node: Node<T>, value: T) -> Node<T> {
        if value > node.value {
            match  node.right{
                None=>{
                    node.right = Some(Box::from(Node::new(value)));
                    node
                },
                Some(n)=>{
                    node.right = Some(Box::from(Node::push(*n, value)));
                    node
                }
            }
        } else if value < node.value {
            match  node.left{
                None=>{
                    node.left = Some(Box::from(Node::new(value)));
                    node
                },
                Some(n)=>{
                    node.left = Some(Box::from(Node::push(*n, value)));
                    node
                }
            }
        } else {
            node
        }
    }

    fn min(&self)-> T {
        match self.left {
            None=>self.value.clone(),
            Some(ref n)=>Node::min(&*n)
        }
    }

    fn max(&self)-> T {
        match self.right {
            None=>self.value.clone(),
            Some(ref n)=>Node::max(&*n)
        }
    }

    fn find(&self,value:T)->bool{
        if value > self.value{
            match self.right {
                None=>false,
                Some(ref n)=>Node::find(&*n,value)
            }
        }else if value < self.value{
            match self.left {
                None=>false,
                Some(ref n)=>Node::find(&*n,value)
            }
        }else{
            true
        }
    }


}

#[derive(Debug, PartialOrd, PartialEq,Clone)]
pub struct Tree<T> {
    root: Child<T>
}

impl<T: PartialOrd+Clone> Tree<T> {
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
            self.root = Some(Box::from(Node::push(*r.unwrap(), value)));
            self
        }
    }

    pub fn min(&self)->T{
        match self.root{
            None=>panic!("root is None!"),
            Some(ref n)=>n.min()
        }
    }

    pub fn max(&self)->T{
        match self.root{
            None=>panic!("root is None!"),
            Some(ref n)=>n.max()
        }
    }

    pub fn find(&self,value:T)->bool {
        match self.root {
            None=>false,
            Some(ref n)=>n.find(value)
        }
    }
}
