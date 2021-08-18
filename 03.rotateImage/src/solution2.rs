#[allow(dead_code)]
fn rotate_image(mut a: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    for i in 0..a.len() {
        // we iterate from i..a.len() because iterating
        // from 0 would swap the elements twice otherwise.
        for j in i..a.len() {
            let temp = a[i][j];

            a[i][j] = a[j][i];
            a[j][i] = temp;
        }
    }

    for column in a.iter_mut() {
        column.reverse();
    }

    a
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn handles_rotation() {
        assert_eq!(
            rotate_image(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
            vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]]
        );
        assert_eq!(rotate_image(vec![vec![1]]), vec![vec![1]],);
        assert_eq!(
            rotate_image(vec![
                vec![10, 9, 6, 3, 7],
                vec![6, 10, 2, 9, 7],
                vec![7, 6, 3, 8, 2],
                vec![8, 9, 7, 9, 9],
                vec![6, 8, 6, 8, 2],
            ]),
            vec![
                vec![6, 8, 7, 6, 10],
                vec![8, 9, 6, 10, 9],
                vec![6, 7, 3, 2, 6],
                vec![8, 9, 8, 9, 3],
                vec![2, 9, 2, 7, 7],
            ]
        );
    }
}
