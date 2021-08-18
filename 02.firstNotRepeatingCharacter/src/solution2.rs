use std::collections::HashMap;

#[allow(dead_code)]
fn first_non_repeating_character(s: String) -> char {
    let mut char_count = HashMap::new();

    // add all chars to the HashSet
    for char in s.chars() {
        char_count.insert(char, char_count.get(&char).unwrap_or(&0) + 1);
    }

    // iterate through, checking to see if no duplicate
    for char in s.chars() {
        if char_count.get(&char).unwrap() == &1 {
            return char;
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
