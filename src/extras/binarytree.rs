#[derive(Debug)]
pub enum Node {
    Leaf(usize),
    Branch { left: Box<Node>, right: Box<Node> },
}

impl Node {
    fn find_node(&mut self, id: usize) -> Option<&mut Node> {
        match self {
            Node::Leaf(node_id) if *node_id == id => Some(self),
            Node::Leaf(node_id) => None,
            Node::Branch { left, right } => {
                match left.find_node(id) {
                    Some(node) => Some(node),
                    None => match right.find_node(id) {
                        Some(node) => Some(node),
                        None => None,
                    }
                }
            }
        }
    }

    pub fn split(&mut self, id: usize, right: usize) {
        if let Some(node) = self.find_node(id) {
            match node {
                Node::Leaf(left) => {
                    *node = Node::Branch {
                        left: Box::new(Node::Leaf(*left)),
                        right: Box::new(Node::Leaf(right)),
                    };
                }
                Node::Branch { left, right } => panic!(),
            }
        }
    }

    pub fn remove(&mut self, node_id: usize) {
    }

    // fn find_parent(&mut self, node_id: usize) -> Option<&mut Node> {
    //     match self {
    //         Node::Leaf(id) => return None,
    //         Node::Branch { left, right } => {
    //             match left.as_mut() {
    //                 Node::Leaf(id) if *id == node_id => return Some(&mut self),
    //                 _ => match right.as_ref() {
    //                     Node::Leaf(id) if *id == node_id => return Some(&mut self),
    //                     _ => {
    //                         match left.find_parent(node_id) {
    //                             Some(p) => Some(p),
    //                             None => right.find_parent(node_id)
    //                         }
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }

}
