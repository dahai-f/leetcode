//! [70. 爬楼梯](https://leetcode-cn.com/problems/climbing-stairs/)

use std::ops::{AddAssign, Deref, DerefMut, Mul};

#[derive(Clone)]
struct Matrix<T: Clone, const R: usize, const C: usize>([[T; C]; R]);

impl<T: Clone + Copy + Default + Mul<Output = T> + AddAssign + From<u8>, const RC: usize>
    Matrix<T, RC, RC>
{
    fn identity() -> Self {
        let mut result = Self::default();
        for rc in 0..RC {
            result[rc][rc] = 1.into();
        }
        result
    }

    fn pow(mut self, mut n: usize) -> Self {
        let mut mul = self.clone();
        self = Matrix::identity();
        while n > 0 {
            if n & 1 != 0 {
                self = &self * &mul;
            }
            n >>= 1;
            if n > 0 {
                mul = &mul * &mul;
            }
        }
        self
    }
}

impl<T: Default + Copy, const R: usize, const C: usize> Default for Matrix<T, R, C> {
    fn default() -> Self {
        Self([[Default::default(); C]; R])
    }
}

impl<T: Clone, const R: usize, const C: usize> Deref for Matrix<T, R, C> {
    type Target = [[T; C]; R];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T: Clone, const R: usize, const C: usize> DerefMut for Matrix<T, R, C> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<
        T: Mul<Output = T> + AddAssign + Default + Copy,
        const R: usize,
        const C: usize,
        const M: usize,
    > Mul<&Matrix<T, M, C>> for &Matrix<T, R, M>
{
    type Output = Matrix<T, R, C>;

    fn mul(self, rhs: &Matrix<T, M, C>) -> Self::Output {
        let mut res = Matrix::default();
        for row in 0..R {
            for col in 0..C {
                let cell = &mut res[row][col];
                for k in 0..M {
                    *cell += self[row][k] * rhs[k][col];
                }
            }
        }
        res
    }
}

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut state = Matrix([[1i32, 1i32]]);
        state = &state * &Matrix([[0, 1], [1, 1]]).pow((n - 1) as usize);
        state[0][1]
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::climb_stairs(1), 1);
    assert_eq!(Solution::climb_stairs(2), 2);
    assert_eq!(Solution::climb_stairs(3), 3);
}
