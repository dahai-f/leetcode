//! [剑指 Offer 10- I. 斐波那契数列](https://leetcode-cn.com/problems/fei-bo-na-qi-shu-lie-lcof/)

use std::ops::{Deref, DerefMut, Mul};

#[derive(Clone)]
struct Matrix([[u64; 2]; 2]);

impl Matrix {
    fn identity() -> Self {
        let mut result = Self::default();
        for rc in 0..2 {
            result[rc][rc] = 1;
        }
        result
    }

    fn pow(mut self, mut n: i32) -> Self {
        let mut result = Matrix::identity();
        while n > 0 {
            if n & 1 != 0 {
                result = &result * &self;
            }
            n >>= 1;
            if n > 0 {
                self = &self * &self;
            }
        }
        result
    }
}

impl Default for Matrix {
    fn default() -> Self {
        Self([[Default::default(); 2]; 2])
    }
}

impl Deref for Matrix {
    type Target = [[u64; 2]; 2];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Matrix {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Mul<&Matrix> for &Matrix {
    type Output = Matrix;

    fn mul(self, rhs: &Matrix) -> Self::Output {
        let mut res = Matrix::default();
        const MOD_NUM: u64 = 1000000007;
        for row in 0..2 {
            for col in 0..2 {
                let cell = &mut res[row][col];
                for k in 0..2 {
                    *cell += self[row][k] * rhs[k][col];
                }
                *cell %= MOD_NUM;
            }
        }
        res
    }
}

impl Solution {
    pub fn fib(n: i32) -> i32 {
        if n == 0 {
            0
        } else if n == 1 {
            1
        } else {
            let mul = Matrix([[0, 1], [1, 1]]);
            let result = &mul.pow(n);
            result[0][1] as i32
        }
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::fib(0), 0);
    assert_eq!(Solution::fib(1), 1);
    assert_eq!(Solution::fib(2), 1);
    assert_eq!(Solution::fib(5), 5);
    assert_eq!(Solution::fib(32), 2178309);
    assert_eq!(Solution::fib(45), 134903163);
    assert_eq!(Solution::fib(76), 598991529);
}
