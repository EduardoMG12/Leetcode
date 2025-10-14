use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::cmp;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode { val, left: None, right: None }
  }
}

fn display_tree(root: &Option<Rc<RefCell<TreeNode>>>) {

    fn get_height(node_opt: &Option<Rc<RefCell<TreeNode>>>) -> usize {
        match node_opt {
            Some(node_rc) => {
                let node = node_rc.borrow();
                1 + cmp::max(get_height(&node.left), get_height(&node.right))
            }
            None => 0,
        }
    }
    let height = get_height(root);
    if height == 0 { println!("Árvore vazia!"); return; }


    let mut levels: Vec<Vec<Option<i32>>> = Vec::new();
    let mut queue = VecDeque::from([root.clone()]);
    for _ in 0..height {
        let mut level_nodes = Vec::new();
        for _ in 0..queue.len() {
            if let Some(node_opt) = queue.pop_front() {
                if let Some(node_rc) = node_opt {
                    let node = node_rc.borrow();
                    level_nodes.push(Some(node.val));
                    queue.push_back(node.left.clone());
                    queue.push_back(node.right.clone());
                } else {
                    level_nodes.push(None);
                    queue.push_back(None);
                    queue.push_back(None);
                }
            }
        }
        levels.push(level_nodes);
    }


    let node_width = 3;
    let mut lines = vec![String::new(); 2 * height - 1];

    for (level_idx, level_nodes) in levels.iter().enumerate() {
        let nodes_count = 1 << level_idx;
        let spacing = (1 << (height - level_idx - 1)) * node_width;

        let initial_padding = spacing / 2 - 1;
        let mut line_cursor = 0;
        
    
        if initial_padding > 0 {
            lines[level_idx * 2].push_str(&" ".repeat(initial_padding));
            line_cursor += initial_padding;
        }

        for (i, val_opt) in level_nodes.iter().enumerate() {
            let val_str = match val_opt {
                Some(val) => format!("{:^width$}", val, width = node_width),
                None => " ".repeat(node_width),
            };
            lines[level_idx * 2].push_str(&val_str);
            line_cursor += node_width;

            if i < nodes_count - 1 {
                lines[level_idx * 2].push_str(&" ".repeat(spacing - node_width));
                line_cursor += spacing - node_width;
            }

        
            if level_idx < height - 1 {
                if val_opt.is_some() {
                    let connector_padding = spacing / 2 - 2;
                    if connector_padding > 0 {
                         lines[level_idx * 2 + 1].push_str(&" ".repeat(connector_padding));
                    }
                    lines[level_idx * 2 + 1].push_str(" /");
                    lines[level_idx * 2 + 1].push_str(&" ".repeat(node_width-2));
                    lines[level_idx * 2 + 1].push_str("\\ ");
                    if connector_padding > 0 {
                         lines[level_idx * 2 + 1].push_str(&" ".repeat(connector_padding));
                    }
                } else {
                    lines[level_idx * 2 + 1].push_str(&" ".repeat(spacing));
                }
            }
        }
    }
    
    for line in lines {
        println!("{}", line);
    }
}

pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    if let Some(node_rc) = &root {
    
        let left_child = node_rc.borrow_mut().left.take();
        let right_child = node_rc.borrow_mut().right.take();      

        node_rc.borrow_mut().left = right_child;
        node_rc.borrow_mut().right = left_child;

        let node = node_rc.borrow();
        invert_tree(node.left.clone());
        invert_tree(node.right.clone());
    }

    root
}

fn main() {
    let root = Rc::new(RefCell::new(TreeNode::new(4)));
    let node2 = Rc::new(RefCell::new(TreeNode::new(2)));
    let node7 = Rc::new(RefCell::new(TreeNode::new(7)));
    root.borrow_mut().left = Some(node2.clone());
    root.borrow_mut().right = Some(node7.clone());
    let node1 = Rc::new(RefCell::new(TreeNode::new(1)));
    let node3 = Rc::new(RefCell::new(TreeNode::new(3)));
    node2.borrow_mut().left = Some(node1.clone());
    node2.borrow_mut().right = Some(node3.clone());
    let node6 = Rc::new(RefCell::new(TreeNode::new(6)));
    let node9 = Rc::new(RefCell::new(TreeNode::new(9)));
    node7.borrow_mut().left = Some(node6.clone());
    node7.borrow_mut().right = Some(node9.clone());


    println!("--- Árvore Original ---");
    let original_root_opt = Some(root.clone());
    display_tree(&original_root_opt);

    println!("\n--- Invertendo a Árvore... ---");
    let inverted_root_opt = invert_tree(original_root_opt);

    println!("\n--- Árvore Invertida ---");
    display_tree(&inverted_root_opt);
}