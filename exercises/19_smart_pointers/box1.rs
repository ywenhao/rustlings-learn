// At compile time, Rust needs to know how much space a type takes up. This
// becomes problematic for recursive types, where a value can have as part of
// itself another value of the same type. To get around the issue, we can use a
// `Box` - a smart pointer used to store data on the heap, which also allows us
// to wrap a recursive type.
//
// The recursive type we're implementing in this exercise is the "cons list", a
// data structure frequently found in functional programming languages. Each
// item in a cons list contains two elements: The value of the current item and
// the next item. The last item is a value called `Nil`.

// TODO: Use a `Box` in the enum definition to make the code compile.
#[derive(PartialEq, Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

// TODO: Create an empty cons list.
fn create_empty_list() -> List {
    List::Nil
}

// TODO: Create a non-empty cons list.
fn create_non_empty_list() -> List {
    List::Cons(
        1,
        Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
    )
}

fn main() {
    println!("This is an empty cons list: {:?}", create_empty_list());
    println!(
        "This is a non-empty cons list: {:?}",
        create_non_empty_list(),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty_list() {
        assert_eq!(create_empty_list(), List::Nil);
    }

    #[test]
    fn test_create_non_empty_list() {
        assert_ne!(create_empty_list(), create_non_empty_list());
    }
}

// #[derive(Debug)]
// struct Node {
//     id: u32,
//     neighbors: Vec<Box<Node>>,
// }

// fn main() {
//     let node1 = Box::new(Node {
//         id: 1,
//         neighbors: vec![],
//     });
//     let node2 = Box::new(Node {
//         id: 2,
//         neighbors: vec![node1],
//     });
//     let node3 = Box::new(Node {
//         id: 3,
//         // use of moved value: `node1`
//         // value used here after move
//         neighbors: vec![node1],
//     });
//     println!("Node 2 neighbors: {:?}", node2.neighbors);
//     println!("Node 3 neighbors: {:?}", node3.neighbors);
// }

// use std::rc::Rc;

// #[derive(Debug)]
// struct Node {
//     id: u32,
//     neighbors: Vec<Rc<Node>>,
// }

// 多个地方使用同一个node节点的时候，需要使用Rc来共享引用。
// fn main() {
//     let node1 = Rc::new(Node {
//         id: 1,
//         neighbors: vec![],
//     });
//     let node2 = Rc::new(Node {
//         id: 2,
//         neighbors: vec![Rc::clone(&node1)],
//     });
//     let node3 = Rc::new(Node {
//         id: 3,
//         neighbors: vec![Rc::clone(&node1)],
//     });
//     println!("Node 2 neighbors: {:?}", node2.neighbors);
//     println!("Node 3 neighbors: {:?}", node3.neighbors);
// }
