mod tree;
mod util;
mod rank_select;
mod balanced_paren;
mod bp_tree;
mod dfuds_tree;
use std::mem::size_of;
use tree::*;
use bp_tree::BPTree;
use dfuds_tree::DFUDSTree;

fn main() {
    let c1 = leaf(1);
    let c2 = node(2, vec![node(3, vec![leaf(4), leaf(5)]), leaf(6)]);
    let c3 = node(7, vec![leaf(8), leaf(9)]);
    let c4 = leaf(10);
    let test_tree = node(0, vec![c1, c2, c3, c4]);
    println!("Tree: {:?}", test_tree);

    // BP
    println!("===BP===");
    let bp_tree = BPTree::new(&test_tree);
    BPTree::print(&bp_tree);
    println!("subtree size of each node");
    for i in 0..11 {
        let sz = BPTree::subtree_size(&bp_tree, i);
        println!("id-{:2} : {:2}", i, sz);
    }

    // DFUDS
    println!("\n===DFUDS===");
    let dfuds_tree = DFUDSTree::new(&test_tree);
    DFUDSTree::print(&dfuds_tree);
    println!("Print Edges");
    for i in 0..11 {
        let vi = DFUDSTree::ith_node(&dfuds_tree, i + 1);
        let d = DFUDSTree::degree(&dfuds_tree, vi);
        for j in 1..d {
            let c = DFUDSTree::ith_child(&dfuds_tree, vi, j);
            println!("{} -- {}", i, c);
        }

    }

}
