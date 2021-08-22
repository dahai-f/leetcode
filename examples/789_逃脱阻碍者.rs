//! [789. 逃脱阻碍者](https://leetcode-cn.com/problems/escape-the-ghosts/)

use std::collections::HashSet;

impl Solution {
    pub fn escape_ghosts(ghosts: Vec<Vec<i32>>, target: Vec<i32>) -> bool {
        let target = (target[0], target[1]);
        if target == (0, 0) {
            return true;
        }
        const DIR: [(i32, i32); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];
        let mut current_ghost_arrived: HashSet<(i32, i32)> = ghosts
            .into_iter()
            .map(|location| (location[0], location[1]))
            .collect();
        let mut next_ghosts_arrived = HashSet::new();
        let mut ghosts_arrived = HashSet::new();
        let mut safe_locations = HashSet::new();
        let mut next_safe_locations = HashSet::new();
        let mut arrived = HashSet::new();
        safe_locations.insert((0, 0));
        loop {
            // ghosts move
            for (x, y) in current_ghost_arrived.drain() {
                for &(dir_x, dir_y) in DIR.iter() {
                    let next_location = (x + dir_x, y + dir_y);
                    if ghosts_arrived.contains(&next_location) {
                        continue;
                    }
                    if next_location == target {
                        return false;
                    }
                    next_ghosts_arrived.insert(next_location);
                    ghosts_arrived.insert(next_location);
                }
            }
            std::mem::swap(&mut current_ghost_arrived, &mut next_ghosts_arrived);

            // I move
            for (x, y) in safe_locations.drain() {
                for &(dir_x, dir_y) in DIR.iter() {
                    let new_location = (x + dir_x, y + dir_y);
                    if !arrived.contains(&new_location) && !ghosts_arrived.contains(&new_location) {
                        if new_location == target {
                            return true;
                        }
                        next_safe_locations.insert(new_location);
                        arrived.insert(new_location);
                    }
                }
            }
            if next_safe_locations.is_empty() {
                return false;
            }
            std::mem::swap(&mut safe_locations, &mut next_safe_locations);
        }
    }
}

struct Solution;

fn main() {
    assert!(!Solution::escape_ghosts(vec![vec![2, 2]], vec![1, 1]));
    assert!(Solution::escape_ghosts(
        vec![
            vec![-1, 0],
            vec![0, 1],
            vec![-1, 0],
            vec![0, 1],
            vec![-1, 0]
        ],
        vec![0, 0]
    ));
    assert!(!Solution::escape_ghosts(
        vec![
            vec![5, 0],
            vec![-10, -2],
            vec![0, -5],
            vec![-2, -2],
            vec![-7, 1]
        ],
        vec![7, 7]
    ));
    assert!(Solution::escape_ghosts(
        vec![vec![1, 0], vec![0, 3]],
        vec![0, 1]
    ));
    assert!(!Solution::escape_ghosts(vec![vec![1, 0]], vec![2, 0]));
    assert!(!Solution::escape_ghosts(vec![vec![2, 0]], vec![1, 0]));
}
