use std::{fs::File, io::Read};

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Point(i32, i32);

pub fn run() {
    let mut contents = String::new();
    let mut file = File::open("ex03_data.txt").expect("could not open file");
    file.read_to_string(&mut contents)
        .expect("could not read data file");

    trim_newline(&mut contents);

    let result = calc_delivered_houses(&contents);

    println!("Result: {}", result);
}

fn calc_delivered_houses(input: &str) -> usize {
    let mut visited_houses: Vec<Point> = vec![Point(0, 0)];
    let mut last_pos: usize = 0;

    for c in input.chars() {
        let new_point = match c {
            '>' => Point(visited_houses[last_pos].0, visited_houses[last_pos].1 + 1),
            '<' => Point(visited_houses[last_pos].0, visited_houses[last_pos].1 - 1),
            '^' => Point(visited_houses[last_pos].0 + 1, visited_houses[last_pos].1),
            'v' => Point(visited_houses[last_pos].0 - 1, visited_houses[last_pos].1),
            _ => {
                println!("caractere {} n reconhecido no input", c);
                panic!("caractere n reconhecido")
            }
        };

        let mut duplicate = false;

        for (i, point) in visited_houses.iter().enumerate() {
            if new_point == *point {
                duplicate = true;
                last_pos = i;
                break;
            }
        }

        if !duplicate {
            visited_houses.push(new_point);
            last_pos = visited_houses.len() - 1;
        }
    }

    let result = visited_houses.len();

    return result;
}

fn trim_newline(s: &mut String) {
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_calc_delivered_houses() {
        assert_eq!(calc_delivered_houses(">"), 2);
        assert_eq!(calc_delivered_houses("^>v<"), 4);
        assert_eq!(calc_delivered_houses("^v^v^v^v^v"), 2);
    }
}
