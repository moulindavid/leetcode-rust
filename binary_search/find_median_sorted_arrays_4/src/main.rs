struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let (nums1, nums2) = if nums1.len() <= nums2.len() {
            (nums1, nums2)
        } else {
            (nums2, nums1)
        };
        let len1 = nums1.len();
        let len2 = nums2.len();

        let total_len = len1 + len2;
        let half = total_len / 2;

        let mut left = 0;
        let mut right = nums1.len();

        loop {
            let middle = (right + left) / 2;
            let nums2_pointer = half - middle;

            let nums1_left = if middle > 0 {
                nums1[middle - 1] as f64
            } else {
                f64::MIN
            };
            let nums1_right = if middle < len1 {
                nums1[middle] as f64
            } else {
                f64::MAX
            };

            let nums2_left = if nums2_pointer > 0 {
                nums2[nums2_pointer - 1] as f64
            } else {
                f64::MIN
            };
            let nums2_right = if nums2_pointer < len2 {
                nums2[nums2_pointer] as f64
            } else {
                f64::MAX
            };

            //If we have a good partition, i.e: all the left values are lower or equal to the right
            //values.
            if nums1_left <= nums2_right && nums2_left <= nums1_right {
                // Odd
                if total_len % 2 == 1 {
                    return nums1_right.min(nums2_right);
                } else {
                    return (nums1_left.max(nums2_left) + nums1_right.min(nums2_right)) / 2f64;
                }
            } else if nums1_left > nums2_right {
                right = middle - 1;
            } else {
                left = middle + 1;
            }
        }
    }
}

fn main() {
    dbg!(Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]));
    dbg!(Solution::find_median_sorted_arrays(vec![1, 3], vec![2]));
}
