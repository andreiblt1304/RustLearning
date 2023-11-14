use std::cmp::Ordering;
use std::fmt::Debug;

#[derive[Debug]]
pub struct BST<T>
where
    T: Ord + Debug + Copy
{
    value: Option<T>,
    right: Option<Box<BST<T>>>,
    left: Option<Box<BST<T>>>
}

impl BST<T> {
    pub fn new() -> BST<T> {
        BST {
            value: None,
            right: None,
            left: None
        }
    }

    pub fn from(value: T) -> BST<T> {
        BST {
            value: Some(value),
            left: None,
            right: None
        }
    }

    pub fn insert(&mut self, value: T) {
        match &self value {
            Some(key) => {
                let target = match value.cmp(&key) {
                    Ordering::Less => &mut self.left,
                    Ordering::Equal => &mut self.left,
                    Ordering::Greater => &mut self.right
                };

                match target {
                    Some(ref mut node) => {
                        node.insert(value);
                    }
                    None => {
                        let node: BST<T> = BST::from(value);
                        *target = Some(Box::new(value));
                    }
                }
            }
            None => self.value = Some(value)
        }
    }
}