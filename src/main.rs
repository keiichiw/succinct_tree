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
const N: i32 = 16;
const B_Size : usize = 2; // B_Size = 1/2 * log N


/// BP (balanced parenthesis)
struct BP {
    p : Bits,
    b : Bits,
    p1: Bits,
}

impl fmt::Display for BP {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let p:String = format!("{:b}", self.p).replace("1", "(").replace("0", ")");
        let b:String = format!("{:32b}", self.b);
        let p1:String = format!("{:b}", self.p1).replace("1", "(").replace("0", ")");
        write!(f, "P:  {}\nB:  {}\nP1: {}", p, b, p1)
    }
}

fn bp_to_P1(b: Bits) -> (Bits, Bits) {
    let mut stack = Vec::new();
    let mut paren = [0 as usize; 2*N as usize];
    let mut far_open = [false; 2*N as usize];
    let sz = format!("{:b}", b).len();
    for i in 0..sz {
        if ((b >> (sz - 1 - i)) & 1) != 0 {
            stack.push(i);
        } else {
            let pre = stack.pop().unwrap() as usize;
            paren[pre as usize] = i as usize;
            paren[i as usize] = pre as usize;
            if pre/B_Size < (i as usize)/B_Size {
                far_open[pre as usize] = true;  // "pre"-th parenthesis is open and far
            }
        }
    }
    println!("Paren: {:?}", paren);
    println!("{:?}", far_open);

    let mut pioneer_parens = [0; 2*N as usize];
    let mut pre_far_open = -1;
    for i in 0..2*N {
        if far_open[i as usize] {
            if pre_far_open == -1 || (0 <= pre_far_open && paren[pre_far_open as usize]/B_Size != paren[i as usize]/B_Size) {
                // i-th paren is opening pioneer
                pioneer_parens[i as usize] = 1;
                // closing paren matching opening pioneer
                pioneer_parens[paren[i as usize]] = -1;
            }
            pre_far_open = i;
        }
    }
    println!("{:?}", pioneer_parens);
    let mut b = 0;
    let mut p1 = 0;
    for p in pioneer_parens.iter() {
        match *p {
            1  => {b = (b<<1) | 1; p1 = (p1<<1) | 1;},
            -1 => {b = (b<<1) | 1; p1 <<= 1;},
            _  => b <<=1
        }
    }
    (b, p1)
}


fn rank(b: u64, p: usize) -> u8 {
    let mut rank = [0 as u8; 65];
    for i in 1..65 {
        rank[i] = rank[i-1] + ((b >> (64 - i)) & 1) as u8;
    }
    rank[p]
}

fn select(b: Bits, x: u32) -> i32 {
    let mut c = 0;
    for i in 0..2*N {
        c += (b >> (2*N - 1 - i)) & 1;
        if c == x as u64 {
            return i
        }
    }
    2*N - 1
}


impl BP {
    fn of_tree_aux(t: Tree) -> Bits {
        match t {
            Tree::Leaf(_) => 0b10,
            Tree::Node(_, children) =>
            {
                let mut bits: Bits = 0b1;
                for child in children {
                    let c_bits = BP::of_tree_aux(child);
                    let c_size = format!("{:b}", c_bits).len();
                    bits = (bits << c_size) | c_bits;
                };
                bits <<= 1;
                bits
            }
        }
    }

    fn of_tree(t: Tree) -> BP {
        let p = BP::of_tree_aux(t);
        let (b, p1) = bp_to_P1(p);
        BP{p: p, b: b, p1: p1}
    }

}

fn find_close(b: Bits, idx: i32) -> i32 {
    let mut stack = Vec::new();
    let mut paren = [0 as usize; 2*N as usize];
    let sz = format!("{:b}", b).len();
    for i in 0..sz {
        if ((b >> (sz - 1 - i)) & 1) != 0 {
            stack.push(i);
        } else {
            let pre = stack.pop().unwrap() as usize;
            paren[pre as usize] = i as usize;
            paren[i as usize] = pre as usize;
            if pre == (idx as usize) {
                return i as i32
            }
        }
    }
    -1
}

fn mu(i: usize, b: Bits, p1: Bits) -> i32 {
    let r = rank(b, i as usize);
    println!("r={}", r);
    let x = find_close(p1, r as i32) + 1; // この +1 は多分必要. find_closeの返り値の最大値は N-1なので
    println!("x={}", x);
    let y = select(b, x as u32);
    println!("y={}", y);
    y
}

fn print_array(arr: &[u8]) {
    print!("[");
    for i in 0..arr.len() {
        print!("{}, ", arr[i])
    }
    println!("]");
}
#[derive(Debug, Clone)]
struct RankIndex{
    b : u64,
    r1: [u8; 2],  // log^2(n) = 36なので、r1 = [0, rank(35)]
    r2: [u8; 24], // 1/2 log(n) = 3 なので、r2[i][j] = rank(i*36 + j * 3) - rank(i*36 + (j-1) * 3)
    r3: [u8; 8]
}
fn create_RankIndex(b: u64) -> RankIndex {
    let mut rank = [0 as u8; 65];
    for i in 1..65 {
        rank[i] = rank[i-1] + ((b >> (64 - i)) & 1) as u8;
    }
    print!("rank: ");
    print_array(&rank);
    let r1 = [0, rank[35]];
    let mut r2 = [0 as u8; 24];
    for i in 0..22 {
        r2[i] = if i < 12 {rank[i*3]} else {rank[i*3] - rank[35]}
    }
    let r3 = [
        0b000000, 0b000001, 0b000101, 0b000110,
        0b010101, 0b010110, 0b011010, 0b011011
    ];

    RankIndex{b: b, r1: r1, r2: r2, r3: r3}
}
fn get_rank(r: &RankIndex, x: usize) -> u8 {
    assert!(x <= 63);
    let a = r.r1[x/36];
    let b = r.r2[x/3];
    if x == 63 {
        return a + b + ((r.b & 1) as u8);
    }
    let idx = ((r.b >> ((63 - ((x / 3) * 3 + 2)) as u64)) as u8) & 0b111;
    let c = (r.r3[idx as usize] >> (((3- ((x + 1) % 3)) % 3) * 2) as u8) & 0b11;
    // println!("{}, {}, {}", a, b, c);
    a + b + c
}

fn main() {
    // let x:u64 = 0b1111111111111111111111111111111111111111111111111111111111111111;
    let x:u64 = 0b1111100001110111010011111010111001010111111110011010111111111111;
    println!("{:b}", x);
    let r = create_RankIndex(x);
    println!("{:?}", r);
    for i in 0..64 {
        let r1 = get_rank(&r, i);
        let r2 = rank(x, i+1);
        if r1 != (r2 as u8) {
        }
        println!("{}: {} {}", i, r1, r2);
        assert!(r1 == (r2 as u8));
    }

}
