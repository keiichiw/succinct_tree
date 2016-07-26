use util;
use tree::Tree;
use balanced_paren::BP;

pub struct BPTree {
    bp: BP
}

impl BPTree {
    pub fn new(t: Tree) -> BPTree {
        let p = util::left_align(BPTree::tree_to_bp(t));
        BPTree{bp: BP::new(p)}
    }

    pub fn ith_node(t: &BPTree, i: u8) -> u8 {
        BP::ith_opening_paren(&t.bp, i)
    }

    /// size of subtree rooted at i-th child
    pub fn subtree_size(t: &BPTree, i: u8) -> u8 {
        let root = BPTree::ith_node(t, i);
        (BP::find_close(&t.bp, root) - root + 1) / 2
    }

    fn tree_to_bp(t: Tree) -> u64 {
        match t {
            Tree::Leaf(_) => 0b10,
            Tree::Node(_, children) =>
            {
                let mut bits: u64 = 0b1;
                for child in children {
                    let c_bits = BPTree::tree_to_bp(child);
                    let c_size = format!("{:b}", c_bits).len();
                    bits = (bits << c_size) | c_bits;
                };
                bits <<= 1;
                bits
            }
        }
    }


}
