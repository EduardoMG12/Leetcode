
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

pub struct Solution;

impl Solution {
    pub fn merge_two_lists(
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        
    
        let mut dummy_head = ListNode::new(0);
        
    
    
        let mut current_tail = &mut dummy_head.next;

    
        while list1.is_some() && list2.is_some() {
        
            let l1_node = list1.as_mut().unwrap();
            let l2_node = list2.as_mut().unwrap();

        
            if l1_node.val <= l2_node.val {
            
            
                let next_node = l1_node.next.take();
                *current_tail = list1.take();
                list1 = next_node;
            } else {
            
                let next_node = l2_node.next.take();
                *current_tail = list2.take();
                list2 = next_node;
            }
            
        
            if let Some(node) = current_tail {
                current_tail = &mut node.next;
            }
        }

    
    
        *current_tail = if list1.is_some() { list1 } else { list2 };

    
        dummy_head.next
    }
}



pub fn vec_to_linked_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
    let mut current = None;

    for &val in vec.iter().rev() {
        let mut new_node = Box::new(ListNode::new(val));
        new_node.next = current;
        current = Some(new_node);
    }
    current
}

pub fn print_linked_list(mut head: &Option<Box<ListNode>>) {
    let mut vals = vec![];
    while let Some(node) = head {
        vals.push(node.val.to_string());
        head = &node.next;
    }
    println!("{}", vals.join(" -> "));
}


fn main() {

    let l1 = vec_to_linked_list(vec![1, 2, 4]);
    let l2 = vec_to_linked_list(vec![1, 3, 4]);

    println!("Lista 1:");
    print_linked_list(&l1);

    println!("Lista 2:");
    print_linked_list(&l2);
    

    let merged_list = Solution::merge_two_lists(l1, l2);

    println!("\nLista Mesclada:");
    print_linked_list(&merged_list);
}