use crate::common::Solution;

impl Solution {
    pub fn find_best_value(arr: Vec<i32>, target: i32) -> i32 {
        let len: i32 = arr.len() as i32;
        let mut sorted_arr: Vec<i32> = arr.clone();
        sorted_arr.sort();

        let min_value = (target as f32 / len as f32).round() as i32;
        let max_value: i32 = *sorted_arr.iter().max().unwrap_or(&0);
        let mut diff: i32 = target;
        let mut result = min_value;

        for value in min_value..max_value + 1 {
            let sum: i32 = sorted_arr
                .iter()
                .map(|x| if *x < value { *x } else { value })
                .sum();
            let current_diff = (target - sum).abs();
            if diff > current_diff {
                diff = current_diff;
                result = value;
            } else {
                break;
            }
        }

        result
    }
}

pub fn test() {
    assert_eq!(Solution::find_best_value(vec![4, 9, 3], 10), 3);
    assert_eq!(Solution::find_best_value(vec![2, 3, 5], 10), 5);
    assert_eq!(
        Solution::find_best_value(vec![60864, 25176, 27249, 21296, 20204], 56803),
        11361,
    );
    assert_eq!(
        Solution::find_best_value(vec![1547, 83230, 57084, 93444, 70879], 71237),
        17422,
    );
}
