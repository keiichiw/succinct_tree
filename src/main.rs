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
const N: i32 = 32;
const B_Size : usize = 2; // B_Size = 1/2 * log N


#[inline(always)]
fn get_bitl(b: Bits, i: u8) -> u8 { // 左からi bit目を取得
    ((b >> (63 - i)) & 1) as u8
}

fn left_align(b: u64) -> u64 {
    let len = format!("{:b}", b).len();
    b << (64 - len)
}




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
    assert!(get_bitl(b, 0) == 1);
    let mut stack = Vec::new();
    let mut paren = [0 as usize; 2*N as usize];
    let mut far_open = [false; 2*N as usize];
    let sz = 64;
    for i in 0..64 {
        if ((b >> (sz - 1 - i)) & 1) != 0 {
            stack.push(i);
        } else {
            if stack.len() == 0 {
                break
            }
            let pre = stack.pop().unwrap() as usize;
            paren[pre as usize] = i as usize;
            paren[i as usize] = pre as usize;
            if pre/B_Size < (i as usize)/B_Size {
                far_open[pre as usize] = true;  // "pre"-th parenthesis is open and far
            }
        }
    }
    // println!("Paren: {:?}", paren);
    // println!("{:?}", far_open);

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
    // println!("{:?}", pioneer_parens);
    let mut b = 0;
    let mut p1 = 0;
    for p in pioneer_parens.iter() {
        match *p {
            1  => {b = (b<<1) | 1; p1 = (p1<<1) | 1;},
            -1 => {b = (b<<1) | 1; p1 <<= 1;},
            _  => b <<=1
        }
    }
    (left_align(b), left_align(p1))
}


fn rank(b: u64, p: usize) -> u8 {
    let mut rank = [0 as u8; 65];
    for i in 1..65 {
        rank[i] = rank[i-1] + ((b >> (64 - i)) & 1) as u8;
    }
    rank[p]
}

fn select(b: Bits, x: u64) -> u8 {
    let mut c = 0;
    for i in 0..2*N {
        c += (b >> (2*N - 1 - i)) & 1;
        if c == x {
            return i as u8
        }
    }
    (2*N - 1) as u8
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

// bit列を
fn find_close_iter(b: Bits, idx: i32) -> u8 {
    let mut stack = Vec::new();
    let mut paren = [0 as usize; 2*N as usize];
    let b = left_align(b);
    let sz = 64;
    println!("{:064b}", b);
    for i in 0..sz {
        if ((b >> (sz - 1 - i)) & 1) != 0 {
            stack.push(i);
        } else {
            if stack.len() == 0 {
                return i-1
            }
            let pre = stack.pop().unwrap() as usize;
            paren[pre as usize] = i as usize;
            paren[i as usize] = pre as usize;
            if pre == (idx as usize) {
                return i as u8
            }
        }
    }
    100
}

fn mu(i: usize, b: Bits, p1: Bits) -> u8 {
    let r = rank(b, i as usize);
    println!("r={}", r);
    let x = find_close_iter(p1, r as i32) + 1; // この +1 は多分必要. find_closeの返り値の最大値は N-1なので
    println!("x={}", x);
    let y = select(b, x as u64);
    println!("y={}", y);
    y
}

fn find_close(bp: BP, p_idx: u8) -> u8 {
    assert!(get_bitl(bp.p, p_idx) == 1);
    if p_idx % 2 == 0 && get_bitl(bp.p, p_idx+1) == 0 {
        // p[p_idx] is not far
        return p_idx + 1
    }
    let mut p_star = p_idx;
    let mut depth_diff = 0;
    while !((get_bitl(bp.p, p_star) & get_bitl(bp.b, p_star)) == 1) {
        if get_bitl(bp.p, p_star) == 1 {
            depth_diff += 1;
        }
        p_star -= 1;
    }
    let mut close_idx = mu(p_star as usize, bp.b, bp.p1) as u8;
    println!("p_star={}, p_star_close={}, diff={}", p_star, close_idx, depth_diff);
    while depth_diff > 0 {
        if get_bitl(bp.p, close_idx) == 0 {
            depth_diff -= 1;
        }
        close_idx -= 1;
    }
    println!("p_star={}, p_star_close={}, diff={}", p_star, close_idx, depth_diff);
    close_idx
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

fn popcount(x : u64) -> u8 {
    let x = format!("{:b}", x);
    let mut a = 0;
    for c in x.chars() {
        if c == '1' {
            a += 1;
        }
    }
    a
}

struct SelectIndex {
    b: u64,
    tree: [u8; 40] // 高さ3の完全3分木 (1 + 3 + 9 + 27 = 40)
}

fn create_SelectIndex(b: u64) -> SelectIndex {
    let tbl = [0, 1, 1, 2, 1, 2, 2, 3]; // 2進表記にしたときの1の個数
    let mut tree = [0 as u8; 40];
    // 4段目
    for i in 0..21 {
        // tree[12 + 1] = b[i*3]...b[i*3+2] のなかの1の個数
        println!("{}", tbl[((b >> (64 - (i+1) * 3)) & 0b111) as usize]);
        tree[13 + i] = tbl[((b >> (64 - (i+1) * 3)) & 0b111) as usize];
    }
    tree[34] = (b & 1) as u8;

    // 3段目
    for i in 0..9 {
        tree[4 + i] = tree[13 + i*3] + tree[13 + i*3 + 1] + tree[13 + i*3 + 2];
    }

    // 2段目
    for i in 0..3 {
        tree[1 + i] = tree[4 + i*3] + tree[4 + i*3 + 1] + tree[4 + i*3 + 2];
    }

    // 1段目
    tree[0] = tree[1] + tree[2] + tree[3];

    assert!(tree[0] == popcount(b));
    println!("{:b}", b);
    print_array(&tree);
    SelectIndex{b: b, tree: tree}
}
fn get_select(s: &SelectIndex, x: u8) -> u8 {
    if s.tree[0] < x {
        return 100
    }
    // 2段目
    let c2 = 1;
    let (acc, second_idx) =
        if x <= s.tree[c2] {
            (0, c2)
        } else if x <= s.tree[c2] + s.tree[c2 + 1] {
            (s.tree[c2], c2 + 1)
        } else {
            (s.tree[c2] + s.tree[c2 + 1], c2 + 2)
        };
        println!("acc : {}", acc);
    // 3段目
    let c3 = 4 + 3 * (second_idx - c2);
    let (acc, third_idx) =
        if x <= acc + s.tree[c3] {
            (acc, c3)
        } else if x <= acc + s.tree[c3] + s.tree[c3 + 1] {
            (acc + s.tree[c3], c3 + 1)
        } else {
            (acc + s.tree[c3] + s.tree[c3 + 1], c3 + 2)
        };
        println!("acc : {}", acc);
    // 4段目 13
    let c4 = 13 + 9 * (second_idx - c2) + (third_idx - c3) * 3;
    let (acc, forth_idx) =
        if x <= acc + s.tree[c4] {
            (acc, c4)
        } else if x <= acc + s.tree[c4] + s.tree[c4 + 1] {
            (acc + s.tree[c4], c4 + 1)
        } else {
            (acc + s.tree[c4] + s.tree[c4 + 1], c4 + 2)
        };

    let c = 27 * (second_idx - c2) + 9 * (third_idx - c3) + 3 * (forth_idx - c4);
    assert!(c < 61);
    let b : u8 = ((s.b >> (61 - c)) & 0b111) as u8;
    let bits = [(b >> 2) & 1, (b >> 1) & 1, b & 1];
    let offset =
        if x == acc + bits[0] {
            0
        } else if x == acc + bits[0] + bits[1] {
            1
        } else {
            2
        };
    let r = 27 * (second_idx - c2) + 9 * (third_idx -c3) + 3 * (forth_idx - c4)+ offset;
    r as u8
}

fn main() {
    let b:Bits = left_align(0b1101110111000011111100011011101010011110111001111111001111111011);
    let sel = create_SelectIndex(b);
    for i in 1..44 {
        let s = select(b, i);
        let t = get_select(&sel, i as u8);
        println!("{} == {}", s, t);
        assert!(s==t);
    }
}
