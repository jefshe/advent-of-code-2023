#[derive(Debug)]
pub struct LRNode<'a> {
    left: &'a str,
    right: &'a str,
}

impl LRNode<'_> {
    pub fn new<'a>(left: &'a str, right: &'a str) -> LRNode<'a> {
        LRNode {
            left,
            right
        }
    }
    pub fn next(&self, l_or_r: char) -> &str {
        match l_or_r {
            'L' => self.left,
            'R' => self.right,
            _ => panic!("invalid path")
        }
    }
}