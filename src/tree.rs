#[derive(Debug, Clone)]
pub enum Tree {
    Leaf(i32),
    Node(i32, Vec<Tree>),
}

pub fn leaf(v: i32) -> Tree {
    Tree::Leaf(v)
}

pub fn node(v: i32, cs: Vec<Tree>) -> Tree {
    Tree::Node(v, cs)
}
