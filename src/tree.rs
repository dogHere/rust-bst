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


    fn value(&self)->T{
        self.value.clone()
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

    fn find(&self,value:T)->Option<Box<&Node<T>>>{
        if value > self.value{
            match self.right {
                None=>None,
                Some(ref n)=>Node::find(&*n,value)
            }
        }else if value < self.value{
            match self.left {
                None=>None,
                Some(ref n)=>Node::find(&*n,value)
            }
        }else{
            Some(Box::from(self))
        }
    }



    fn delete(mut child:Child<T>,value:T)->Child<T>{
        match child{
            None=>None,
            Some(mut r)=>{
                if value > r.value(){
                    r.right =
                        Node::delete(r.right.clone(),value)  ;
                    Some(r)
                }else if value < r.value() {
                    r.left  = Node::delete(r.left.clone() ,value);
                    Some(r)
                }else{
                    if r.right.is_none(){
                        r.left
                    }else if r.left.is_none() {
                        r.right
                    }else{
                        r.value = r.min();
                        r.right = Node::delete(r.right.clone(),r.value.clone());
                        Some(r)
                    }
                }
            }
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

    pub fn find(&self,value:T)->Option<Box<&Node<T>>> {
        match self.root {
            None=>None,
            Some(ref n)=>n.find(value)
        }
    }

    pub fn exists(&self,value:T)->bool{
        match self.find(value){
            None=>false,
            _=>true
        }
    }

    pub fn delete(mut self,value:T)->Tree<T>{
        self.root = Node::delete(self.root,value);
        self
    }
}
