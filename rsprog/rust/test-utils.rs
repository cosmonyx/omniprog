fn main() {
    prog::main();
}

//////////////////////////////////////////////////////////////////////////////
//
// It demonstrates a singly linked list using structs in Rust.
//
// -- It shows defining a Node struct.
// -- It shows adding nodes at the end of the list.
// -- It shows printing the list.
//

#[cfg(prog010)]
mod prog {

struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

fn append(head: &mut Option<Box<Node>>, value: i32) {

    if head.is_none() {
        *head = Some(Box::new(Node { value, next: None }));
        return;
    }

    let mut cur = head.as_mut();

    while let Some(node) = cur {
        if node.next.is_none() {
            node.next = Some(Box::new(Node { value, next: None }));
            return;
        }
        cur = node.next.as_mut();
    }
}

fn print_list(head: &Option<Box<Node>>) {
    let mut cur = head;

    while let Some(node) = cur {
        print!("{} -> ", node.value);
        cur = &node.next;
    }

    println!("None");
}

pub fn main() {

    let mut head: Option<Box<Node>> = None;

    append(&mut head, 1);
    append(&mut head, 2);
    append(&mut head, 3);
    append(&mut head, 4);

    println!("Linked list:");
    print_list(&head);
}

}
