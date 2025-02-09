#[derive(Debug)]
struct Node<T> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

// Require T to implement PartialOrd (for comparisons) and Display (for printing)
impl<T: PartialOrd + std::fmt::Display> Node<T> {
    /// Create a new Node with the given value.
    fn new(value: T) -> Self {
        Node {
            value,
            left: None,
            right: None,
        }
    }

    /// Insert a new value into the binary search tree.
    fn insert(&mut self, new_value: T) {
        if new_value < self.value {
            match self.left {
                Some(ref mut left_child) => {
                    left_child.insert(new_value);
                }
                None => {
                    self.left = Some(Box::new(Node::new(new_value)));
                }
            }
        } else {
            match self.right {
                Some(ref mut right_child) => {
                    right_child.insert(new_value);
                }
                None => {
                    self.right = Some(Box::new(Node::new(new_value)));
                }
            }
        }
    }

    /// Perform an in-order traversal of the tree and print each value.
    fn inorder_traversal(&self) {
        if let Some(ref left) = self.left {
            left.inorder_traversal();
        }
        println!("{}", self.value);
        if let Some(ref right) = self.right {
            right.inorder_traversal();
        }
    }
}

fn main() {
    // Create the root of the binary search tree.
    let mut root = Node::new(10);

    // Insert some values into the tree.
    root.insert(5);
    root.insert(15);
    root.insert(3);
    root.insert(7);
    root.insert(12);
    root.insert(18);

    // Perform an in-order traversal.
    println!("In-order traversal of the tree:");
    root.inorder_traversal();
}
