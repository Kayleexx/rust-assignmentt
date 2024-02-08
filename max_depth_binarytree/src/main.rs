//a function that returns the maximum depth of the tree in binary tree

use std::io;

struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(val: i32) -> Self {
        TreeNode { val, left: None, right: None }
    }
}

fn max_depth(root: &Option<Box<TreeNode>>) -> i32 {
    match root {
        Some(node) => {
            let left_depth = max_depth(&node.left);
            let right_depth = max_depth(&node.right);
            left_depth.max(right_depth) + 1
        }
        None => 0,
    }
}

fn build_tree() -> Option<Box<TreeNode>> {
    println!("Enter the value of the root node (-1 for empty node):");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let val: i32 = input.trim().parse().expect("Invalid input");

    if val == -1 {
        return None; // empty node
    }

    let mut root = TreeNode::new(val);
    println!("Enter the left subtree of {}:", val);
    root.left = build_tree();
    println!("Enter the right subtree of {}:", val);
    root.right = build_tree();

    Some(Box::new(root))
}

fn main() {
    let root = build_tree();
    let depth = max_depth(&root);
    println!("Maximum depth of the binary tree: {}", depth);
}
