use std::collections::HashMap;

#[allow(dead_code)]
fn is_crypt_solution(crypt: [String; 3], solution: Vec<[char; 2]>) -> bool {
    let mapped_solution: HashMap<char, char> = solution
        .iter()
        .map(|solution| (solution[0], solution[1]))
        .collect();

    let mapped_crypt: Vec<String> = crypt
        .iter()
        .map(|crypt| crypt.chars().map(|char| mapped_solution[&char]).collect())
        .collect();

    let mapped_crypt_numbers: Vec<u32> = mapped_crypt
        .iter()
        .map(|crypt| {
            crypt
                .chars()
                .map(|char| char.to_digit(10).unwrap())
                .fold(0, |acc, elem| acc * 10 + elem)
        })
        .collect();

    let a = mapped_crypt_numbers[0];
    let b = mapped_crypt_numbers[1];
    let c = mapped_crypt_numbers[2];

    // validate that solution[0] + solution[1] == solution[2]
    if a + b != c {
        return false;
    }

    // validate that there is no leading empty zeroes
    for crypt in mapped_crypt {
        if crypt.starts_with('0') && crypt.len() > 1 {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn handles_valid_solution() {
        assert_eq!(
            is_crypt_solution(
                [
                    String::from("SEND"),
                    String::from("MORE"),
                    String::from("MONEY")
                ],
                vec![
                    ['O', '0'],
                    ['M', '1'],
                    ['Y', '2'],
                    ['E', '5'],
                    ['N', '6'],
                    ['D', '7'],
                    ['R', '8'],
                    ['S', '9']
                ]
            ),
            true
        );

        assert_eq!(
            is_crypt_solution(
                [String::from("A"), String::from("A"), String::from("A")],
                vec![['A', '0']]
            ),
            true
        )
    }

    #[test]
    fn handles_leading_zero() {
        assert_eq!(
            is_crypt_solution(
                [
                    String::from("TEN"),
                    String::from("TWO"),
                    String::from("ONE")
                ],
                vec![['O', '1'], ['T', '0'], ['W', '9'], ['E', '5'], ['N', '4']]
            ),
            false
        );
    }
}
