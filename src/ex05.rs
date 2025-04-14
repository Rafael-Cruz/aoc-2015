use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

pub fn run() {
    let file = File::open("ex05_data.txt").expect("could not open file");
    let reader = BufReader::new(file);
    let mut result: u32 = 0;

    for line in reader.lines() {
        if str_is_nice(&line.unwrap()) {
            result = result + 1;
        }
    }

    println!("Nice strings: {}", result);
}

fn str_has_three_vowels(string: &str) -> bool {
    string.chars()
        .filter(|c| VOWELS.contains(c))
        .collect::<Vec<char>>()
        .len() >= 3
}

fn str_has_double_letter(string: &str) -> bool {
    let mut result = false;

    for slice in string.chars().collect::<Vec<_>>().windows(2) {
        if slice[0] == slice[1] {
            result = true;
            break;
        }
    }

    result
}

fn str_contain_forbidden_string(string: &str) -> bool {
    string.contains("ab")
    || string.contains("cd")
    || string.contains("pq")
    || string.contains("xy")
}

fn str_is_nice(string: &str) -> bool {
    str_has_three_vowels(string)
    && str_has_double_letter(string)
    && !str_contain_forbidden_string(string)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_str_has_three_vowels() {
        assert_eq!(str_has_three_vowels("ugknbfddgicrmopn"), true);
        assert_eq!(str_has_three_vowels("aaa"), true);
        assert_eq!(str_has_three_vowels("dvszwmarrgswjxmb"), false);
    }

    #[test]
    fn test_str_has_double_letter() {
        assert_eq!(str_has_double_letter("ugknbfddgicrmopn"), true);
        assert_eq!(str_has_double_letter("aaa"), true);
        assert_eq!(str_has_double_letter("jchzalrnumimnmhp"), false);
    }

    #[test]
    fn test_str_contain_forbidden_string() {
        assert_eq!(str_contain_forbidden_string("haegwjzuvuyypxyu"), true);
        assert_eq!(str_contain_forbidden_string("ugknbfddgicrmopn"), false);
    }

    #[test]
    fn test_str_is_nice() {
        assert_eq!(str_is_nice("ugknbfddgicrmopn"), true);
        assert_eq!(str_is_nice("aaa"), true);
        assert_eq!(str_is_nice("jchzalrnumimnmhp"), false);
        assert_eq!(str_is_nice("haegwjzuvuyypxyu"), false);
        assert_eq!(str_is_nice("dvszwmarrgswjxmb"), false);
    }
}