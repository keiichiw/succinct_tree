#[inline(always)]
pub fn get_bitl(b: &u64, i: u8) -> u8 { // 左からi bit目を取得
    ((b >> (63 - i)) & 1) as u8
}

pub fn left_align(b: u64) -> u64 {
    let len = format!("{:b}", b).len();
    b << (64 - len)
}
