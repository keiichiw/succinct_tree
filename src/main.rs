use std::fmt;

#[derive(Debug, Clone)]
enum Tree {
    Leaf(i32),
    Node(i32, Vec<Tree>),
}

fn leaf(v: i32) -> Tree {
    Tree::Leaf(v)
}
fn node(v: i32, cs: Vec<Tree>) -> Tree {
    Tree::Node(v, cs)
}

type Bits = u64;



/// BP (balanced parenthesis)
struct BP {
    bits :Bits
}

impl fmt::Display for BP {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s:String = format!("{:b}", self.bits);
        write!(f, "{}", s.replace("1", "(").replace("0", ")"))
    }
}
impl BP {
    fn of_tree(t: Tree) -> BP {
        match t {
            Tree::Leaf(_) => BP {bits: 0b10},
            Tree::Node(_, children) =>
            {
                let mut bits: Bits = 0b1;
                for child in children {
                    let c_bits = BP::of_tree(child).bits;
                    let c_size = format!("{:b}", c_bits).len();
                    bits = (bits << c_size) | c_bits;
                };
                bits <<= 1;
                BP {bits: bits}
            }
        }
    }

}

/// DFUDS (depth first unary degree sequence)
struct DFUDS {
    bits : Bits
}

impl fmt::Display for DFUDS {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s:String = format!("{:b}", self.bits);
        write!(f, "{}", s.replace("1", "(").replace("0", ")"))
    }
}

impl DFUDS {
    fn of_tree_aux(t: Tree, b : Bits) -> Bits {
        match t {
            Tree::Leaf(_) => b<<1,
            Tree::Node(_, children) =>
            {
                let size = children.len();
                let mut bits: Bits = (b<< (size+1)) | (((1<<size)-1) << 1);
                for child in children {
                    bits = DFUDS::of_tree_aux(child, bits);
                };
                bits
            }
        }
    }
    fn of_tree(t: Tree) -> DFUDS {
        DFUDS{bits: DFUDS::of_tree_aux(t, 0b1)}
    }
}

fn main() {
    let t = node(3, vec![leaf(1), leaf(2)]);
    let bp = BP::of_tree(t.clone());
    let dfuds = DFUDS::of_tree(t.clone());
    println!("Tree:  {:?}", t);
    println!("BP:    {}", bp);
    println!("DFUDS: {}", dfuds);

    let t = node(1, vec![node(2, vec![leaf(3), leaf(4), leaf(5)]), node(6, vec![leaf(7), leaf(8)])]);
    let bp = BP::of_tree(t.clone());
    let dfuds = DFUDS::of_tree(t.clone());
    println!("Tree:  {:?}", t);
    println!("BP:    {}", bp);
    println!("DFUDS: {}", dfuds);

}
