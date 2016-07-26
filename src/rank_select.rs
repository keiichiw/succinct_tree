pub struct RankSelectIndex {
    pub b: u64,

    /* Index for Rank */
    pub r1: [u8; 2],  // log^2(n) = 36なので、r1 = [0, rank(35)]
    pub r2: [u8; 24], // 1/2 log(n) = 3 なので、r2[i][j] = rank(i*36 + j * 3) - rank(i*36 + (j-1) * 3)

    /* Index for Select */
    pub tree: [u8; 40] // 高さ3の完全3分木 (1 + 3 + 9 + 27 = 40)
}

impl RankSelectIndex {

    pub fn new(b: u64) -> Self {
        let (r1, r2) = Self::create_rank_index(b);
        let tree = Self::create_select_index(b);
        RankSelectIndex{b: b, r1: r1, r2: r2, tree: tree}
    }

    pub fn rank(r: &Self, x: u8) -> u8 {
        let a = r.r1[(x/36) as usize];
        let b = r.r2[(x/3)  as usize];
        if x == 63 {
            return a + b + ((r.b & 1) as u8);
        }
        let idx = ((r.b >> ((63 - ((x / 3) * 3 + 2)) as u64)) as u8) & 0b111;

        let rank_table : [u8; 8] = [
            0b000000, 0b000001, 0b000101, 0b000110,
            0b010101, 0b010110, 0b011010, 0b011011
        ];
        let c = (rank_table[idx as usize] >> (((3- ((x + 1) % 3)) % 3) * 2) as u8) & 0b11;
        a + b + c
    }

    pub fn select(s: &Self, x: u8) -> u8 {
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
        let bit_offset = 27 * (second_idx - c2) + 9 * (third_idx - c3) + 3 * (forth_idx - c4);
        if bit_offset == 63 {
            return bit_offset as u8;
        }
        assert!(bit_offset <= 61);
        let b : u8 = ((s.b >> (61 - bit_offset)) & 0b111) as u8;
        let bits = [(b >> 2) & 1, (b >> 1) & 1, b & 1];
        let fifth_index =
            if x == acc + bits[0] {
                0
            } else if x == acc + bits[0] + bits[1] {
                1
            } else {
                2
            };
        (bit_offset + fifth_index) as u8
    }

    fn create_rank_index(b: u64) -> ([u8; 2], [u8; 24]) {
        let mut rank = [0 as u8; 65];
        for i in 1..65 {
            rank[i] = rank[i-1] + ((b >> (64 - i)) & 1) as u8;
        }
        let r1 = [0, rank[35]];
        let mut r2 = [0 as u8; 24];
        for i in 0..22 {
            r2[i] = if i < 12 {rank[i*3]} else {rank[i*3] - rank[35]}
        }
        (r1, r2)
    }

    fn create_select_index(b: u64) -> [u8; 40] {
        let tbl = [0, 1, 1, 2, 1, 2, 2, 3]; // tbl[i] = iを2進表記にしたときの1の個数
        let mut tree = [0 as u8; 40];
        // 4段目
        for i in 0..21 {
            // tree[12 + 1] = b[i*3]...b[i*3+2] のなかの1の個数
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

        tree
    }

}


#[cfg(test)]
mod tests {

    use rank_select::RankSelectIndex;

    fn naive_rank(b: u64, x: u8) -> u8 {
        let mut acc:u8 = 0;
        for i in 0..x+1 {
            acc += ((b >> (63 - i)) & 1) as u8;
        }
        acc
    }

    fn naive_select(b: u64, x: u8) -> u8 {
        let mut acc:u8 = 0;
        for i in 0..64 {
            acc += ((b >> (63 - i)) & 1) as u8;
            if acc == x {
                return i
            }
        }
        64
    }

    #[test]
    fn test_rank() {
        let bits = 0b1011101010110101000100010010111011111100100111010111110111001011;
        let idx = RankSelectIndex::new(bits);
        for i in 0..64 {
            let expect = naive_rank(bits, i);
            let ans = RankSelectIndex::rank(&idx, i);
            assert_eq!(expect, ans);
        }
    }

    #[test]
    fn test_select() {
        let bits = 0b1011101010110101000100010010111011111100100111010111110111001011;
        let pop = naive_rank(bits, 63);
        let idx = RankSelectIndex::new(bits);
        for i in 1..pop+1 {
            let expect = naive_select(bits, i);
            let ans = RankSelectIndex::select(&idx, i);
            assert_eq!(expect, ans);
        }
    }
}
