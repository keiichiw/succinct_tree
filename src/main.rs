mod tree;
mod util;
mod rank_select;
mod balanced_paren;
mod bp_tree;
use std::mem::size_of;
use tree::*;
use bp_tree::BPTree;

fn main() {
    let c1 = leaf(1);
    let c2 = node(2, vec!(node(3, vec!(leaf(4), leaf(5))), leaf(6)));
    let c3 = node(7, vec!(leaf(8), leaf(9)));
    let c4 = leaf(10);
    let test_tree = node(0, vec!(c1, c2, c3, c4));
    println!("Tree: {:?}", test_tree);
    let bptree = BPTree::new(test_tree);
    println!("size_of(BPTree) = {}", size_of::<BPTree>());
    for i in 0..11 {
        let sz = BPTree::subtree_size(&bptree, i);
        println!("subtree size of child-{} = {:2}", i, sz);
    }
}
