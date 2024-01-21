// Program to implement Linked List

#[derive(Debug, PartialEq)]
struct ListNode {
    data: u32,
    next: Option<Box<ListNode>>,
}

#[derive(Debug)]
struct LinkedList {
    start_node: Option<Box<ListNode>>,
}

impl LinkedList {
    fn add_item(&mut self, data: u32) {
        if self.start_node == None {
            self.start_node = Some(Box::new(ListNode { data, next: None }));
            return;
        }
        let current_node = self.start_node;
        match current_node.next {
            Some(current_node) => current_node,
            None => current_node.next = Some(new_node)
        }
        
    }
}

pub fn main() {
    let mut linked_list = LinkedList { start_node: None };
    linked_list.add_item(32);
    dbg!(linked_list);
}
