fn main() {
    
}

pub struct LinkedList<T> {
    pub value: Option<T>,
    pub next: Option<Box<LinkedList<T>>>
}

impl LinkedList<i32> {
    // **This is the constructor which return a new instance of the linked list with None values
    // ** Usage: let list = mini_linked_list::LinkedList::<i32>::new();
    pub fn new() -> LinkedList<i32> {
        LinkedList { value: None, next: None }
    }

    pub fn push_left(mut self, x: i32) {
        let node = LinkedList::<i32> {
            value: Some(x),
            next: Some(Box::new(self))
        };

        self = node;
    }
}