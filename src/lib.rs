

#[cfg(test)]
mod tests {
    use crate::Tree;
    use rand::RngCore;

    #[test]
    fn tree_new() {
        let tree: Tree<i32> = Tree::new();
    }

    #[test]
    fn tree_insert() {
        let mut tree: Tree<u32> = Tree::new();
        let mut r = rand::thread_rng();
        for i in 0..1000 {
            tree.insert(r.next_u32())
        }
    }

    #[test]
    fn tree_inorder() {
        let mut tree: Tree<u32> = Tree::new();
        let mut r = rand::thread_rng();
        for i in 0..1000 {
            tree.insert(r.next_u32())
        }
        let mut prev = std::u32::MIN;
        for i in tree.inorder_iter() {
            assert!(i > &prev);
            prev = *i;
        }
    }
}


struct Node<V: PartialOrd> {
    left: Option<Box<Node<V>>>,
    right: Option<Box<Node<V>>>,
    value: V,
}

impl<V: PartialOrd > Node<V> {
    fn new(value: V) -> Node<V> {
        Node {
            left: None,
            right: None,
            value,
        }
    }
}


struct Tree<V: PartialOrd > {
    root: Option<Box<Node<V>>>
}


impl<V: PartialOrd > Tree<V> {
    pub fn new() -> Tree<V> {
        Tree {
            root: None,
        }
    }

    pub fn insert(&mut self, value: V) {
        let insertion_value = &value;
        match &mut self.root {
            None => {
                self.root = Some(Box::new(Node::new(value)));
            }
            Some(ref mut v) => {
                let mut current = v;
                'outer: loop {
                    if &current.value > insertion_value {
                        match current.left {
                            Some(ref mut v) => {
                                current = v;
                                continue;
                            }
                            None => {
                                current.left = Some(Box::new(Node::new(value)));
                                break 'outer;
                            }
                        }
                    } else if &current.value < insertion_value {
                        match current.right {
                            Some(ref mut v) => {
                                current = v;
                                continue;
                            }
                            None => {
                                current.right = Some(Box::new(Node::new(value)));
                                break 'outer;
                            }
                        }
                    } else {
                        break 'outer;
                    }
                }
            }
        }
    }

    pub fn preorder_iter(&self) -> PreorderIterator<V> {
        let root = &self.root;
        PreorderIterator {
            current: root,
            stack: Vec::new(),
        }
    }


    pub fn inorder_iter(&self) -> InorderIterator<V> {
        let root = &self.root;
        InorderIterator {
            current: root,
            stack: Vec::new(),
        }
    }
}


struct PreorderIterator<'a, V: PartialOrd> {
    current: &'a Option<Box<Node<V>>>,
    stack: Vec<&'a Box<Node<V>>>,
}

impl<'a, V: PartialOrd + std::fmt::Debug> Iterator for PreorderIterator<'a, V> {
    type Item = &'a V;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match &self.current {
                None => {
                    match self.stack.pop() {
                        None => { return None; }
                        Some(v) => {
                            self.current = &v.right;
                        }
                    }
                }
                Some(ref v) => {
                    self.stack.push(v);
                    self.current = &v.left;

                    return Some(&v.value);
                }
            }
        }

        None
    }
}


struct InorderIterator<'a, V: PartialOrd > {
    current: &'a Option<Box<Node<V>>>,
    stack: Vec<&'a Box<Node<V>>>,
}

impl<'a, V: PartialOrd> Iterator for InorderIterator<'a, V> {
    type Item = &'a V;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match &self.current {
                None => {
                    return match self.stack.pop() {
                        None => { None }
                        Some(v) => {
                            self.current = &v.right;
                            Some(&v.value)
                        }
                    };
                }
                Some(ref v) => {
                    self.stack.push(v);
                    self.current = &v.left;
                }
            }
        }

        None
    }
}