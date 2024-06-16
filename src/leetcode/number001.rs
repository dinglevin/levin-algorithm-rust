pub(crate) struct Solution;

impl Solution {
    pub fn two_sum_v1(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let len = nums.len();
        for i in 0..len {
            for j in (i + 1)..len {
                if nums[i] + nums[j] == target {
                    return vec![i as i32, j as i32];
                }
            }
        }
        return vec![0, 0]
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum_v1() {
        assert_eq!(Solution::two_sum_v1(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(Solution::two_sum_v1(vec![3, 2, 4], 6), vec![1, 2]);
        assert_eq!(Solution::two_sum_v1(vec![3, 3], 6), vec![0, 1]);
    }
}