pub fn transpose<T: Clone>(matrix: Vec<Vec<T>>) -> Vec<Vec<T>> {
    if matrix.is_empty() {
        return vec![];
    }

    let rows = matrix.len();
    let cols = matrix[0].len();

    let mut result: Vec<Vec<T>> = vec![vec![]; cols];

    for i in 0..rows {
        for j in 0..cols {
            if let Some(item) = matrix[i].get(j) {
                result.get_mut(j).unwrap().push(item.clone());
            } else {
                // if we are the last row it is fine to be short
                if i < rows - 1 {
                    panic!("Only the last row can be short");
                }
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transpose() {
        let input = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let expected = vec![vec![1, 4], vec![2, 5], vec![3, 6]];

        assert_eq!(transpose(input), expected);
    }

    #[test]
    fn test_transpose_with_short_final_row() {
        let input = vec![vec![1, 2, 3], vec![4, 5]];
        let expected = vec![vec![1, 4], vec![2, 5], vec![3]];
        assert_eq!(transpose(input), expected);
    }

    #[test]
    #[should_panic]
    fn test_panic_on_short_row() {
        let input = vec![vec![1, 2, 3], vec![4, 5], vec![7, 8, 9]];
        transpose(input);
    }
}
