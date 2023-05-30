trait Iter<'a> {
    type Item;
    type Iter: Iterator<Item = Self::Item> + 'a;

    fn iter(&'a self) -> Self::Iter;
}

struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    fn new() -> Self {
        List { head: None }
    }
}

impl<'a, T: 'a> Iter<'a> for List<T> {
    type Item = &'a T;
    type Iter = ListIter<'a, T>;

    fn iter(&'a self) -> Self::Iter {
        ListIter { next: self.head.as_ref().map(|node| &**node) }
    }
}

struct ListIter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for ListIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next {
            Some(node) => {
                self.next = node.next.as_ref().map(|node| &**node);
                Some(&node.elem)
            }
            None => None,
        }
    }
}

fn main() {
    let mut list = List::new();
    list.head = Some(Box::new(Node { elem: 1, next: None }));
    list.head = Some(Box::new(Node { elem: 2, next: list.head }));
    list.head = Some(Box::new(Node { elem: 3, next: list.head }));
    list.head = Some(Box::new(Node { elem: 4, next: list.head }));
    list.head = Some(Box::new(Node { elem: 5, next: list.head }));

    for i in list.iter() {
        println!("{}", i);
    }
}

// Output: 5
//         4
//         3
//         2
//         1
