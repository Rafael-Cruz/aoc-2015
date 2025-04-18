use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn run() {
    let total_box = calc_total_box();
    let total_ribbon = calc_total_ribbon();

    println!("Total box: {}, Total ribbon: {}", total_box, total_ribbon);
}

pub fn calc_total_box() -> u32 {
    let file = File::open("ex02_data.txt").expect("could not open file");
    let reader = BufReader::new(file);
    let mut result: u32 = 0;

    for line in reader.lines() {
        result = result + calc_box(&line.unwrap());
    }

    return result;
}

pub fn calc_total_ribbon() -> u32 {
    let file = File::open("ex02_data.txt").expect("could not open file");
    let reader = BufReader::new(file);
    let mut result: u32 = 0;

    for line in reader.lines() {
        result = result + calc_ribbon(&line.unwrap());
    }

    return result;
}

pub fn calc_ribbon(input: &str) -> u32 {
    let nums = parse_input(input);

    let l = nums.get(0).unwrap();
    let w = nums.get(1).unwrap();
    let h = nums.get(2).unwrap();

    let mut sorted = vec![l, w, h];
    sorted.sort();

    let ribbon_feet = sorted[0] + sorted[0] + sorted[1] + sorted[1];
    let bow_feet = l * w * h;

    return ribbon_feet + bow_feet;
}

pub fn calc_box(input: &str) -> u32 {
    let nums = parse_input(input);

    let l = nums.get(0).unwrap();
    let w = nums.get(1).unwrap();
    let h = nums.get(2).unwrap();

    let l_x_w = l * w;
    let w_x_h = w * h;
    let l_x_h = l * h;

    let smallest = smallest(&l_x_w, &w_x_h, &l_x_h);

    return (2 * l_x_w) + (2 * w_x_h) + (2 * l_x_h) + smallest;
}

fn parse_input(input: &str) -> Vec<u32> {
    let tokens = input.split('x');

    // println!("tokens: {:?}", tokens);

    let nums = tokens
        .map(|num| num.parse())
        .collect::<Result<Vec<u32>, _>>()
        .expect(format!("input string parse error. input: {}", input).as_str());

    return nums;
}

fn smallest<'a>(l: &'a u32, w: &'a u32, h: &'a u32) -> &'a u32 {
    let sizes = [l, w, h];
    let smallest = sizes
        .iter()
        .reduce(|acc, e| if e < acc { e } else { acc })
        .unwrap();
    return smallest;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_calc_box() {
        assert_eq!(calc_box("2x3x4"), 58);
        assert_eq!(calc_box("1x1x10"), 43);
    }

    #[test]
    fn test_calc_ribbon() {
        assert_eq!(calc_ribbon("2x3x4"), 34);
        assert_eq!(calc_ribbon("1x1x10"), 14);
    }
}
