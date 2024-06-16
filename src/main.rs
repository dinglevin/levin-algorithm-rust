use leetcode::number001::Solution;

mod leetcode {
    pub mod number001;
}

fn main() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let result = Solution::two_sum_v1(nums, target);
    println!("result: {:?}", result);
}
