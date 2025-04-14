use std::{
    collections::HashMap, fs::File, io::{BufRead, BufReader}
};

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

fn str_has_double_letter(string: &str) -> bool {
    let mut pairs: HashMap<String, (u32, usize)> = HashMap::new();

    for (i, slice) in string.chars().collect::<Vec<_>>().windows(2).enumerate() {
        pairs
            .entry(String::from_iter(slice))
            .and_modify(|value| {
                if i > value.1 + 1 {
                    value.0 = value.0 + 1;
                }
            })
            .or_insert((1, i));
    }

    return pairs.iter().any(|(_, (count, _))| *count >= 2);
}

fn str_has_triplet_letter(string: &str) -> bool {
    let mut result = false;

    for slice in string.chars().collect::<Vec<_>>().windows(3) {
        if slice[0] == slice[2] {
            result = true;
            break;
        }
    }

    result
}

fn str_is_nice(string: &str) -> bool {
    str_has_double_letter(string) && str_has_triplet_letter(string)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_str_has_double_letter() {
        assert_eq!(str_has_double_letter("qjhvhtzxzqqjkmpb"), true);
        assert_eq!(str_has_double_letter("xyxy"), true);
        assert_eq!(str_has_double_letter("aabcdefgaa"), true);
        assert_eq!(str_has_double_letter("xxyxx"), true);
        assert_eq!(str_has_double_letter("aaa"), false);
    }

    #[test]
    fn test_str_has_triplet_letter() {
        assert_eq!(str_has_triplet_letter("abcdefeghi"), true);
        assert_eq!(str_has_triplet_letter("xyx"), true);
        assert_eq!(str_has_triplet_letter("aaa"), true);
        assert_eq!(str_has_triplet_letter("ieodomkazucvgmuy"), true);
        assert_eq!(str_has_triplet_letter("uurcxstgmygtbstg"), false);
        assert_eq!(str_has_triplet_letter("aabcdefgaa"), false);
    }

    #[test]
    fn test_str_is_nice() {
        assert_eq!(str_is_nice("qjhvhtzxzqqjkmpb"), true);
        assert_eq!(str_is_nice("xxyxx"), true);
        assert_eq!(str_is_nice("uurcxstgmygtbstg"), false);
        assert_eq!(str_is_nice("ieodomkazucvgmuy"), false);
    }
}