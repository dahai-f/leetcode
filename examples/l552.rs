//! [552. 学生出勤记录 II](https://leetcode-cn.com/problems/student-attendance-record-ii/)

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

impl State {
    fn count(&self) -> i32 {
        ((self.l + self.al + self.ll + self.all + self.p + self.ap) % MOD) as i32
    }
}

impl Solution {
    pub fn check_record(n: i32) -> i32 {
        let mut pre_state = State {
            l: 1,
            al: 0,
            ll: 0,
            all: 0,
            p: 1,
            ap: 1,
        };
        for _ in 1..n {
            let mut new_state = State::default();
            // if p
            new_state.ap += pre_state.ap + pre_state.al + pre_state.all;
            new_state.p += pre_state.p + pre_state.l + pre_state.ll;
            // if l
            new_state.l += pre_state.p;
            new_state.al += pre_state.ap;
            new_state.ll += pre_state.l;
            new_state.all += pre_state.al;
            // if a
            new_state.ap += pre_state.p + pre_state.l + pre_state.ll;

            new_state.p %= MOD;
            new_state.ap %= MOD;

            std::mem::swap(&mut pre_state, &mut new_state);
        }
        pre_state.count()
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::check_record(2), 8);
    assert_eq!(Solution::check_record(1), 3);
    assert_eq!(Solution::check_record(10101), 183236316);
}
