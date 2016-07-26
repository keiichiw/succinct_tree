use util::*;
use rank_select::RankSelectIndex;

const N: i32 = 32;
const B_SIZE : usize = 2; // B_SIZE = 1/2 * log N

pub struct BP {
    pub p : u64,
    pub p_idx : RankSelectIndex,
    pub b : RankSelectIndex,
    pub p1: u64
}

impl BP {

    pub fn new(p : u64) -> Self {
        let (raw_b, p1) = Self::calc_p1(p);
        let p_idx = RankSelectIndex::new(p);
        let b = RankSelectIndex::new(raw_b);
        BP{p: p, p_idx: p_idx, b: b, p1: p1}
    }

    #[allow(dead_code)]
    pub fn of_string(s : String) -> Self {
        let mut p = 0;
        for c in s.chars() {
            p <<= 1;
            if c == '(' {
                p += 1;
            } else {
                assert!(c == ')');
            }
        }
        Self::new(left_align(p))
    }


    pub fn find_close(bp: &Self, p_idx: u8) -> u8 {
        assert!(get_bitl(&bp.p, p_idx) == 1);
        if p_idx % 2 == 0 && get_bitl(&bp.p, p_idx+1) == 0 {
            // p[p_idx] is not far
            return p_idx + 1
        }
        let mut p_star = p_idx;
        let mut depth_diff = 0;

        while !((get_bitl(&bp.p, p_star) & get_bitl(&bp.b.b, p_star)) == 1) {
            if get_bitl(&bp.p, p_star) == 1 {
                depth_diff += 1;
            }
            p_star -= 1;
        }
        let mut close_idx = Self::mu(p_star, &bp.b, bp.p1) as u8;
        while depth_diff > 0 {
            if get_bitl(&bp.p, close_idx) == 0 {
                depth_diff -= 1;
            }
            close_idx -= 1;
        }
        close_idx
    }

    pub fn ith_opening_paren(bp: &Self, i: u8) -> u8 {
        RankSelectIndex::select(&bp.p_idx, i+1)
    }

    pub fn print(bp: &Self) {
        let mut c = 0;
        for i in 0..64 {
            if get_bitl(&bp.p, i) == 0 {
                if c == 0 {
                    break
                }
                print!(")");
                c -= 1;
            } else {
                print!("(");
                c += 1;
            }
        }
        println!("");
    }

    // private functions
    fn mu(i: u8, b: &RankSelectIndex, p1: u64) -> u8 {
        let r = RankSelectIndex::rank(b, i) - 1; // - 1が謎
        let x = Self::naive_find_close(p1, r) + 1; // この +1 は多分必要. find_closeの返り値の最大値は N-1なので
        let y = RankSelectIndex::select(b, x as u8);
        y
    }

    fn calc_p1(b: u64) -> (u64, u64) {
        assert!(get_bitl(&b, 0) == 1);
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
                if pre/B_SIZE < (i as usize)/B_SIZE {
                    far_open[pre as usize] = true;  // "pre"-th parenthesis is open and far
                }
            }
        }

        let mut pioneer_parens = [0; 2*N as usize];
        let mut pre_far_open = -1;
        for i in 0..2*N {
            if far_open[i as usize] {
                if pre_far_open == -1 || (0 <= pre_far_open && paren[pre_far_open as usize]/B_SIZE != paren[i as usize]/B_SIZE) {
                    // i-th paren is opening pioneer
                    pioneer_parens[i as usize] = 1;
                    // closing paren matching opening pioneer
                    pioneer_parens[paren[i as usize]] = -1;
                }
                pre_far_open = i;
            }
        }
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

    pub fn naive_find_close(b: u64, idx: u8) -> u8 {
        let mut stack = Vec::new();
        let mut paren = [0 as usize; 2*N as usize];
        let b = left_align(b);
        let sz = 64;
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

}

#[cfg(test)]
mod tests {
    use balanced_paren::BP;

    fn test_find_close_aux(paren: &str) {
        let bp = BP::of_string(paren.to_string());
        for i in 0..paren.len() {
            if (bp.p >> (63 - i)) == 1 { // Case of open paren
                let expect = BP::naive_find_close(bp.p, i as u8);
                let ans = BP::find_close(&bp, i as u8);
                assert_eq!(expect, ans);
            }

        }
    }

    #[test]
    fn test_find_close1() {
        let paren = "((()()())(()()))";
        test_find_close_aux(paren);
    }

    #[test]
    fn test_find_close2() {
        let paren = "(((((((((((((((((((((((((((((((())))))))))))))))))))))))))))))))";
        test_find_close_aux(paren);
    }

    #[test]
    fn test_find_close3() {
        let paren = "(((()(()(((()))()()()()(()()()()((((()((())(())((())))))))))))))";
        test_find_close_aux(paren);
    }

}
