mod queue;
mod graph;

use graph::Graph;
use graph::GraphNodeRef;
use queue::Queue;
use std::rc::Rc;

fn main() {

    let mut g: Graph<i32> = Graph::new();
    let root = g.add_root_node(1);
    let n2 = g.add_child(Rc::clone(&root), 2);
    let n3 = g.add_child(Rc::clone(&n2), 3);

    g.add_child(Rc::clone(&n2), 4);
    g.add_child(Rc::clone(&n3), 6);
    g.add_child(Rc::clone(&n2), 7);

    bfs(root);
}

fn bfs(s: GraphNodeRef<i32>) {

    let mut queue: Queue<i32> = Queue::new();

    s.borrow_mut().visited = true;
    println!("node {} visited", s.borrow().value);

    queue.enqueue(s);

    loop {
        if queue.len() == 0 {
            break;
        }

        let u = queue.dequeue();

        match u {
            Some(_u) => {
                let mut _u = _u.borrow_mut();
                let distance = _u.d;
                let neighbors = &_u.childs;
                for neighbor in neighbors {
                    if (neighbor.borrow()).visited == false {
                        println!("node {} visited", (neighbor.borrow()).value);
                        neighbor.borrow_mut().visited = true;
                        neighbor.borrow_mut().d = distance + 1;
                        queue.enqueue(Rc::clone(&neighbor));
                    }
                }
                _u.visited = true;
            },
            None => {}
        }
    }
}

