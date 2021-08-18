#[allow(dead_code)]
fn first_duplicate(a: Vec<i32>) -> i32 {
    let mut min_second_index = a.len();

    for i in 0..a.len() {
        for j in i + 1..a.len() {
            if a[i] == a[j] {
                min_second_index = min_second_index.min(j);
            }
        }
    }

    if min_second_index == a.len() {
        return -1;
    }

    a[min_second_index]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handles_multiple_duplicates() {
        assert_eq!(first_duplicate(vec![2, 1, 3, 5, 3, 2]), 3);
        assert_eq!(first_duplicate(vec![2, 1, 3, 5, 2, 3]), 2);
        assert_eq!(first_duplicate(vec![8, 4, 6, 2, 6, 4, 7, 9, 5, 8]), 6);
    }

    #[test]
    fn handles_one_element() {
        assert_eq!(first_duplicate(vec![2, 2]), 2);
        assert_eq!(first_duplicate(vec![5, 5, 5, 5, 5]), 5);
    }

    #[test]
    fn handles_no_duplicate() {
        assert_eq!(first_duplicate(vec![]), -1);
        assert_eq!(first_duplicate(vec![2, 4, 3, 5, 1]), -1);
        assert_eq!(first_duplicate(vec![1]), -1);
        assert_eq!(first_duplicate(vec![2, 1]), -1);
    }
}
