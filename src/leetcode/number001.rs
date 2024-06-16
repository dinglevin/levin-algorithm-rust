/**
 * 给定一个整数数组 nums 和一个整数目标值 target，请你在该数组中找出 和为目标值 target  的那 两个 整数，并返回它们的数组下标。
 * 你可以假设每种输入只会对应一个答案。但是，数组中同一个元素在答案里不能重复出现。
 * 你可以按任意顺序返回答案。
 *
 * 示例 1：
 * 输入：nums = [2,7,11,15], target = 9
 * 输出：[0,1]
 * 解释：因为 nums[0] + nums[1] == 9 ，返回 [0, 1] 。
 *
 * 示例 2：
 * 输入：nums = [3,2,4], target = 6
 * 输出：[1,2]
 *
 * 示例 3：
 * 输入：nums = [3,3], target = 6
 * 输出：[0,1]
 *
 * 提示：
 * 2 <= nums.length <= 10^4
 * -10^9 <= nums[i] <= 10^9
 * -10^9 <= target <= 10^9
 * 只会存在一个有效答案
 *
 * 进阶：你可以想出一个时间复杂度小于 O(n2) 的算法吗？
 */
use std::collections::HashMap;

pub(crate) struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn two_sum_v1(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let len = nums.len();
        for i in 0..len {
            for j in (i + 1)..len {
                if nums[i] + nums[j] == target {
                    return vec![i as i32, j as i32];
                }
            }
        }
        return vec![0, 0];
    }

    #[allow(dead_code)]
    pub fn two_sum_v2(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();

        for (i, &num) in nums.iter().enumerate() {
            let complement = target - num;

            if let Some(&index) = map.get(&complement) {
                return vec![index, i as i32];
            }

            map.insert(num, i as i32);
        }

        return vec![0, 0];
    }

    #[allow(dead_code)]
    pub fn two_sum_v3(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut cloned_nums = nums.clone();
        cloned_nums.sort();

        let mut left = 0;
        let mut right = nums.len() - 1;
        while left < right {
            let sum = cloned_nums[left] + cloned_nums[right];
            if sum == target {
                let i = nums.iter().position(|&x| x == cloned_nums[left]);
                let j = nums.iter().rposition(|&x| x == cloned_nums[right]);
                return vec![i.unwrap() as i32, j.unwrap() as i32];
            } else if sum < target {
                left += 1;
            } else {
                right -= 1;
            }
        }

        return vec![0, 0];
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

    #[test]
    fn test_two_sum_v2() {
        assert_eq!(Solution::two_sum_v2(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(Solution::two_sum_v2(vec![3, 2, 4], 6), vec![1, 2]);
        assert_eq!(Solution::two_sum_v2(vec![3, 3], 6), vec![0, 1]);
    }

    #[test]
    fn test_two_sum_v3() {
        assert_eq!(Solution::two_sum_v3(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(Solution::two_sum_v3(vec![3, 2, 4], 6), vec![1, 2]);
        assert_eq!(Solution::two_sum_v3(vec![3, 3], 6), vec![0, 1]);
    }
}
