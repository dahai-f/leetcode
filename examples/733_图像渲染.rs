//! [733. 图像渲染](https://leetcode-cn.com/problems/flood-fill/)

impl Solution {
    pub fn flood_fill(mut image: Vec<Vec<i32>>, sr: i32, sc: i32, new_color: i32) -> Vec<Vec<i32>> {
        let (sr, sc) = (sr as usize, sc as usize);
        let ini_color = image[sr][sc];
        if ini_color == new_color {
            return image;
        }
        Self::repaint(&mut image, sr, sc, ini_color, new_color);
        image
    }

    fn repaint(
        image: &mut Vec<Vec<i32>>,
        row: usize,
        column: usize,
        ini_color: i32,
        new_color: i32,
    ) {
        if row >= image.len() || column >= image[0].len() {
            return;
        }
        let p = &mut image[row][column];
        if *p != ini_color {
            return;
        }

        *p = new_color;
        Self::repaint(image, row, column.wrapping_sub(1), ini_color, new_color);
        Self::repaint(image, row, column + 1, ini_color, new_color);
        Self::repaint(image, row.wrapping_sub(1), column, ini_color, new_color);
        Self::repaint(image, row + 1, column, ini_color, new_color);
    }
}

struct Solution;

fn main() {
    assert_eq!(
        Solution::flood_fill(vec![vec![1, 1, 1], vec![1, 1, 0], vec![1, 0, 1]], 1, 1, 2),
        vec![vec![2, 2, 2], vec![2, 2, 0], vec![2, 0, 1]]
    );
}
