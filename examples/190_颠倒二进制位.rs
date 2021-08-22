//! [190. 颠倒二进制位](https://leetcode-cn.com/problems/reverse-bits/)

impl Solution {
    pub fn reverse_bits(mut x: u32) -> u32 {
        let mask_0 = 0b0101_0101_0101_0101_0101_0101_0101_0101u32;
        let mask_1 = 0b0011_0011_0011_0011_0011_0011_0011_0011u32;
        let mask_2 = 0b0000_1111_0000_1111_0000_1111_0000_1111u32;
        let mask_3 = 0b0000_0000_1111_1111_0000_0000_1111_1111u32;
        let mask_4 = 0b0000_0000_0000_0000_1111_1111_1111_1111u32;
        x = ((x & mask_0) << 1) | (x >> 1 & mask_0);
        x = ((x & mask_1) << 2) | (x >> 2 & mask_1);
        x = ((x & mask_2) << 4) | (x >> 4 & mask_2);
        x = ((x & mask_3) << 8) | (x >> 8 & mask_3);
        x = ((x & mask_4) << 16) | (x >> 16 & mask_4);
        x
    }
}

struct Solution;

fn main() {}
