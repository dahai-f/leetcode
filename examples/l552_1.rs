//! [552. 学生出勤记录 II](https://leetcode-cn.com/problems/student-attendance-record-ii/)

use std::ops::{Deref, DerefMut, Mul};

const MOD: u64 = 1_000_000_007;

#[derive(Default)]
struct State {
    l: u64,
    al: u64,
    ll: u64,
    all: u64,
    p: u64,
    ap: u64,
}

impl Mul<Matrix> for State {
    type Output = State;

    fn mul(self, rhs: Matrix) -> Self::Output {
        let mut res = Self::default();
        res.l += self.l * rhs[0][0]
            + self.al * rhs[1][0]
            + self.ll * rhs[2][0]
            + self.all * rhs[3][0]
            + self.p * rhs[4][0]
            + self.ap * rhs[5][0];
        res.l %= MOD;
        res.al += self.l * rhs[0][1]
            + self.al * rhs[1][1]
            + self.ll * rhs[2][1]
            + self.all * rhs[3][1]
            + self.p * rhs[4][1]
            + self.ap * rhs[5][1];
        res.al %= MOD;
        res.ll += self.l * rhs[0][2]
            + self.al * rhs[1][2]
            + self.ll * rhs[2][2]
            + self.all * rhs[3][2]
            + self.p * rhs[4][2]
            + self.ap * rhs[5][2];
        res.ll %= MOD;
        res.all += self.l * rhs[0][3]
            + self.al * rhs[1][3]
            + self.ll * rhs[2][3]
            + self.all * rhs[3][3]
            + self.p * rhs[4][3]
            + self.ap * rhs[5][3];
        res.all %= MOD;
        res.p += self.l * rhs[0][4]
            + self.al * rhs[1][4]
            + self.ll * rhs[2][4]
            + self.all * rhs[3][4]
            + self.p * rhs[4][4]
            + self.ap * rhs[5][4];
        res.p %= MOD;
        res.ap += self.l * rhs[0][5]
            + self.al * rhs[1][5]
            + self.ll * rhs[2][5]
            + self.all * rhs[3][5]
            + self.p * rhs[4][5]
            + self.ap * rhs[5][5];
        res.ap %= MOD;
        res
    }
}

#[derive(Default, Clone)]
struct Matrix([[u64; 6]; 6]);

impl Deref for Matrix {
    type Target = [[u64; 6]; 6];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Matrix {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Mul for &Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut res = Matrix::default();
        for row in 0..6 {
            for col in 0..6 {
                let cell = &mut res[row][col];
                for k in 0..6 {
                    *cell += self[row][k] * rhs[k][col];
                }
                *cell %= MOD;
            }
        }
        res
    }
}

impl State {
    fn count(&self) -> i32 {
        ((self.l + self.al + self.ll + self.all + self.p + self.ap) % MOD) as i32
    }
}

impl Solution {
    pub fn check_record(n: i32) -> i32 {
        let state = State {
            p: 1,
            ..Default::default()
        };
        const MAT: Matrix = Matrix([
            [0, 0, 1, 0, 1, 1],
            [0, 0, 0, 1, 0, 1],
            [0, 0, 0, 0, 1, 1],
            [0, 0, 0, 0, 0, 1],
            [1, 0, 0, 0, 1, 1],
            [0, 1, 0, 0, 0, 1],
        ]);
        let mat = Self::pow(MAT.clone(), n);
        (state * mat).count()
    }

    fn pow(mut mat: Matrix, mut pow: i32) -> Matrix {
        assert!(pow >= 1);
        pow -= 1;
        let mut res = mat.clone();
        while pow > 0 {
            if pow & 1 == 1 {
                res = &res * &mat;
            }
            pow >>= 1;
            mat = &mat * &mat;
        }
        res
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::check_record(2), 8);
    assert_eq!(Solution::check_record(1), 3);
    assert_eq!(Solution::check_record(10101), 183236316);
}
