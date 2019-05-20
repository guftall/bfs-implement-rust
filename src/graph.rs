use std::rc::Rc;
use std::cell::RefCell;

pub type GraphNodeRef<T> = Rc<RefCell<GraphNode<T>>>;

pub struct Graph<T> {
    pub root: Option<GraphNodeRef<T>>
}

pub struct GraphNode<T> {
    pub value: T,
    pub childs: Vec<GraphNodeRef<T>>,
    pub visited: bool,
    pub d: i32
}

impl<T> GraphNode<T> {
    pub fn new(value: T) -> GraphNode<T> {
        GraphNode::<T> {
            value: value, 
            childs: vec![],
            d: 0,
            visited: false
        }
    }
}

impl<T> Graph<T> {
    pub fn new() -> Graph<T> {
        Graph::<T> {
            root: None
        }
    }

    pub fn add_root_node(&mut self, value: T) -> GraphNodeRef<T> {
        let n = GraphNode::new(value);
        let r_node = Rc::new(RefCell::new(n));
        self.root = Some(Rc::clone(&r_node));
        r_node
    }

    pub fn add_child(&mut self, node: GraphNodeRef<T>, value: T) -> GraphNodeRef<T> {
        let n = GraphNode::new(value);
        let r_node = Rc::new(RefCell::new(n));
        node.borrow_mut().childs.push(r_node.clone());
        r_node
    }
}