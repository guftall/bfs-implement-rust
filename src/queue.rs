use crate::graph::GraphNodeRef;

pub struct Queue<T> {
    elements: std::vec::Vec<GraphNodeRef<T>>
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            elements: vec![]
        }
    }
    pub fn enqueue(&mut self, element: GraphNodeRef<T>) {
        self.elements.push(element);
    }
    pub fn dequeue(&mut self) -> Option<GraphNodeRef<T>> {
        self.elements.pop()
    }
    pub fn len(&self) -> usize {
        self.elements.len()
    }
}