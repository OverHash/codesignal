#[allow(dead_code)]
fn rotate_image(a: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let length = a.len();

    let mut output = vec![vec![0; length]; length];

    for (i, row) in a.iter().enumerate() {
        for j in 0..length {
            output[j][length - 1 - i] = row[j];
        }
    }

    output
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
