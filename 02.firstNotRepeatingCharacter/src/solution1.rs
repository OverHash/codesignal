#[allow(dead_code)]
fn first_non_repeating_character(s: String) -> char {
    for i in 0..s.len() {
        let mut has_duplicate = false;

        for j in 0..s.len() {
            if i == j {
                continue;
            }
            if s.chars().nth(i).unwrap() == s.chars().nth(j).unwrap() {
                has_duplicate = true;
                break;
            }
        }

        if !has_duplicate {
            return s.chars().nth(i).unwrap();
        }
    }

    '_'
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn handles_no_duplicates() {
        assert_eq!(first_non_repeating_character(String::from("abacabad")), 'c');
        assert_eq!(first_non_repeating_character(String::from("z")), 'z');
        assert_eq!(first_non_repeating_character(String::from("bcb")), 'c');
        assert_eq!(
            first_non_repeating_character(String::from(
                "abcdefghijklmnopqrstuvwxyziflskecznslkjfabe"
            )),
            'd'
        );
        assert_eq!(
            first_non_repeating_character(String::from("bcccccccccccccyb")),
            'y'
        );
        assert_eq!(
            first_non_repeating_character(String::from(
                "xdnxxlvupzuwgigeqjggosgljuhliybkjpibyatofcjbfxwtalc"
            )),
            'd'
        );
        assert_eq!(
            first_non_repeating_character(String::from(
                "ngrhhqbhnsipkcoqjyviikvxbxyphsnjpdxkhtadltsuxbfbrkof"
            )),
            'g'
        );
    }

    #[test]
    fn handles_duplicate() {
        assert_eq!(
            first_non_repeating_character(String::from("abacabaabacaba")),
            '_'
        );
        assert_eq!(
            first_non_repeating_character(String::from("bcccccccb")),
            '_'
        );
        assert_eq!(first_non_repeating_character(String::from("zzz")), '_');
    }
}
