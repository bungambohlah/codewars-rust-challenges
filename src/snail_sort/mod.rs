pub fn run(matrix: &[Vec<i32>]) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();

    if matrix.is_empty() || (!matrix.is_empty() && matrix[0].is_empty()) {
        return result;
    }

    let mut top_row = 0;
    let mut bottom_row = matrix.len() - 1;
    let mut left_col = 0;
    let mut right_col = matrix[0].len() - 1;

    while top_row <= bottom_row && left_col <= right_col {
        // Traverse top row
        for i in left_col..=right_col {
            result.push(matrix[top_row][i]);
        }
        top_row += 1;

        // Traverse right column
        for i in top_row..=bottom_row {
            result.push(matrix[i][right_col]);
        }
        if right_col > 0 {
            right_col -= 1;
        }

        if top_row <= bottom_row {
            // Traverse bottom row
            for i in (left_col..=right_col).rev() {
                result.push(matrix[bottom_row][i]);
            }
            bottom_row -= 1;
        }

        if left_col <= right_col {
            // Traverse left column
            for i in (top_row..=bottom_row).rev() {
                result.push(matrix[i][left_col]);
            }
            left_col += 1;
        }
    }

    result
}

#[test]
fn sample_test1() {
    let square = &[vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let expected = vec![1, 2, 3, 6, 9, 8, 7, 4, 5];
    assert_eq!(run(square), expected);
}

#[test]
fn sample_test2() {
    let square = &[vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5]];
    let expected = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    assert_eq!(run(square), expected);
}

#[test]
fn sample_test3() {
    let square: &[Vec<i32>; 1] = &[Vec::new()];
    let expected = Vec::new();
    assert_eq!(run(square), expected, "Failed with empty input");
}

#[test]
fn sample_test4() {
    let square = &[vec![1]];
    let expected = vec![1];
    assert_eq!(run(square), expected);
}
