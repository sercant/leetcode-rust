use crate::common::Solution;

impl Solution {
    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        let mut result = 0;
        let mut transposed_grid: Vec<Vec<i32>> = vec![vec![0; grid.len()]; grid[0].len()];
        for (i, row) in grid.iter().enumerate() {
            let mut last_val = 0;
            for (j, &val) in row.iter().enumerate() {
                if last_val != val {
                    result += 1;
                }

                last_val = val;

                transposed_grid[j][i] = val;
            }
            result += last_val;
        }

        for row in transposed_grid.iter() {
            let mut last_val = 0;
            for &val in row.iter() {
                if last_val != val {
                    result += 1;
                }

                last_val = val;
            }
            result += last_val;
        }

        result
    }
}

pub fn test() {
    assert_eq!(
        Solution::island_perimeter(vec![
            vec![0, 1, 0, 0],
            vec![1, 1, 1, 0],
            vec![0, 1, 0, 0],
            vec![1, 1, 0, 0]
        ]),
        16
    );
    assert_eq!(
        Solution::island_perimeter(vec![
            vec![1, 1, 1, 1],
            vec![0, 1, 1, 1],
            vec![1, 1, 1, 1],
            vec![1, 1, 1, 1]
        ]),
        18
    );
    assert_eq!(Solution::island_perimeter(vec![vec![0, 0]]), 0);

    assert_eq!(Solution::island_perimeter(vec![vec![1, 0]]), 4);
}
