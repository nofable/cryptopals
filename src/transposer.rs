use crate::errors::CryptopalsError;

pub fn transpose<T: Clone>(matrix: &[&[T]]) -> Result<Vec<Vec<T>>, CryptopalsError> {
    if matrix.is_empty() {
        return Ok(vec![]);
    }

    let rows = matrix.len();
    let cols = matrix[0].len();

    let mut result = vec![vec![]; cols];

    for (i, row) in matrix.iter().enumerate() {
        let len = row.len();

        if len < cols && i < rows - 1 {
            return Err(CryptopalsError::MisshapedMatrix {
                reason: format!(
                    "Row {} is too short. Only last row can be shorter than the rest.",
                    i
                ),
            });
        } else if len > cols {
            return Err(CryptopalsError::MisshapedMatrix {
                reason: format!("Row {} is longer than the first row of the matrix", i),
            });
        }

        for (j, item) in row.iter().enumerate() {
            result[j].push(item.clone());
        }
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transpose() {
        let input = [vec![1, 2, 3], vec![4, 5, 6]];
        let expected = vec![vec![1, 4], vec![2, 5], vec![3, 6]];
        let block_refs: Vec<&[u8]> = input.iter().map(|b| b.as_slice()).collect();
        assert_eq!(transpose(&block_refs).unwrap(), expected);
    }

    #[test]
    fn test_transpose_with_short_final_row() {
        let input = [vec![1, 2, 3], vec![4, 5]];
        let expected = vec![vec![1, 4], vec![2, 5], vec![3]];
        let block_refs: Vec<&[u8]> = input.iter().map(|b| b.as_slice()).collect();
        assert_eq!(transpose(&block_refs).unwrap(), expected);
    }

    #[test]
    fn test_error_on_short_row() {
        let input = [vec![1, 2, 3], vec![4, 5], vec![7, 8, 9]];
        let block_refs: Vec<&[u8]> = input.iter().map(|b| b.as_slice()).collect();
        assert!(matches!(
            transpose(&block_refs).unwrap_err(),
            CryptopalsError::MisshapedMatrix {
                reason
            } if reason.contains("too short")
        ));
    }

    #[test]
    fn test_error_on_long_row() {
        let input = [vec![1, 2, 3], vec![4, 5, 6, 7], vec![8, 9]];
        let block_refs: Vec<&[u8]> = input.iter().map(|b| b.as_slice()).collect();
        assert!(matches!(
            transpose(&block_refs).unwrap_err(),
            CryptopalsError::MisshapedMatrix {
                reason
            } if reason.contains("longer")
        ));
    }
}
