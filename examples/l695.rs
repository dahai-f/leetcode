//! [695. 岛屿的最大面积](https://leetcode-cn.com/problems/max-area-of-island/)

impl Solution {
    pub fn max_area_of_island(mut grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut area = 0;
        for row in 0..m {
            for col in 0..n {
                area = Self::dfs(&mut grid, row, col).max(area);
            }
        }
        area
    }

    fn dfs(grid: &mut [Vec<i32>], row: usize, col: usize) -> i32 {
        if row >= grid.len() || col >= grid[0].len() {
            return 0;
        }

        let val = &mut grid[row][col];
        if *val == 0 {
            return 0;
        }
        *val = 0;

        1 + Self::dfs(grid, row, col.wrapping_sub(1))
            + Self::dfs(grid, row, col + 1)
            + Self::dfs(grid, row.wrapping_sub(1), col)
            + Self::dfs(grid, row + 1, col)
    }
}

struct Solution;

fn main() {
    assert_eq!(
        Solution::max_area_of_island(vec![
            vec![0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
            vec![0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0],
            vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0],
        ]),
        6
    );
}
