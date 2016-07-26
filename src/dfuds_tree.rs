use util;
use tree::Tree;
use balanced_paren::BP;
use rank_select::RankSelectIndex;

/**
 * DFUDS (depth first unary degree sequence)
 */

pub struct DFUDSTree {
    dfuds: BP,
    inv: RankSelectIndex,
}

impl DFUDSTree {
    pub fn new(t: &Tree) -> Self {
        let p = util::left_align(Self::tree_to_bp(t.clone(), 0b1));
        let inv = RankSelectIndex::new(p ^ 0xffffffffffffffff);
        DFUDSTree {
            dfuds: BP::new(p),
            inv: inv,
        }
    }

    pub fn ith_node(t: &Self, i: u8) -> u8 {
        RankSelectIndex::select(&t.inv, i - 1)
    }

    pub fn node_id(t: &Self, v: u8) -> u8 {
        RankSelectIndex::rank(&t.inv, v - 1) + 1
    }

    pub fn degree(t: &Self, v: u8) -> u8 {
        let r = RankSelectIndex::rank(&t.inv, v);
        let s = RankSelectIndex::select(&t.inv, r + 1);
        s - v
    }

    pub fn ith_child(t: &Self, v: u8, i: u8) -> u8 {
        let r = RankSelectIndex::rank(&t.inv, v);
        let s = RankSelectIndex::select(&t.inv, r + 1);
        let vj = BP::find_close(&t.dfuds, s - i) + 1;
        Self::node_id(&t, vj) - 1
    }

    pub fn print(t: &Self) {
        BP::print(&t.dfuds);
    }

    fn tree_to_bp(t: Tree, b: u64) -> u64 {
        match t {
            Tree::Leaf(_) => b << 1,
            Tree::Node(_, children) => {
                let size = children.len();
                let mut bits: u64 = (b << (size + 1)) | (((1 << size) - 1) << 1);
                for child in children {
                    bits = Self::tree_to_bp(child, bits);
                }
                bits
            }
        }
    }
}
