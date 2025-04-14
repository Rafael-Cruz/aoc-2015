use std::fs;
use regex::Regex;

const LIGHTS_LEN: usize = 1000;

type LightsArray = Vec<Vec<bool>>;

#[derive(Debug)]
enum Command {
    TurnOn,
    TurnOff,
    Toggle
}

#[derive(Debug)]
struct Instruction {
    command: Command,
    initial_coordinates: (usize, usize),
    final_coordinates: (usize, usize)
}

pub fn run() {
    let mut lights: LightsArray = vec![vec![false; LIGHTS_LEN]; LIGHTS_LEN];

    // //  Init tests
    // print_lights(&lights);

    // let instr = Instruction {
    //     command: Command::TurnOn,
    //     initial_coordinates: (8, 8),
    //     final_coordinates: (9, 9)
    // };

    // execute_instruction(&instr, &mut lights);
    // print_lights(&lights);

    // let instr = Instruction {
    //     command: Command::TurnOn,
    //     initial_coordinates: (4, 4),
    //     final_coordinates: (5, 5)
    // };

    // execute_instruction(&instr, &mut lights);
    // print_lights(&lights);

    // let instr = Instruction {
    //     command: Command::Toggle,
    //     initial_coordinates: (0, 4),
    //     final_coordinates: (9, 4)
    // };

    // execute_instruction(&instr, &mut lights);
    // print_lights(&lights);
    // //  Finish tests


    let str_data = fs::read_to_string("ex06_data.txt").expect("could not open file");
    let instructions = parse_instructions(&str_data);

    for instr in instructions {
        execute_instruction(&instr, &mut lights);
    }

    let count = get_lit_lights_qty(&lights);

    println!("Number of lit lights: {}", count);
}

fn get_lit_lights_qty(lights: &LightsArray) -> u32 {
    let mut count = 0;
    for i in 0..LIGHTS_LEN {
        for j in 0..LIGHTS_LEN {
            if lights[i][j] {
                count = count + 1;
            }
        }
    }
    return count;
}

fn print_lights(lights: &LightsArray) {
    print!("\n");
    for i in 0..LIGHTS_LEN {
        for j in 0..LIGHTS_LEN {
            print!(" {} ", if lights[i][j] { "☼" } else { "•" })
        }
        print!("\n");
    }
    print!("\n");
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    let re = Regex::new(r"(.*) (\d{1,3},\d{1,3}) .* (\d{1,3},\d{1,3})").unwrap();
    
    re.captures_iter(input)
    .map(|caps| {
        let (_, [command, coord1, coord2]) = caps.extract();
        (command, coord1.split_once(',').unwrap(), coord2.split_once(',').unwrap())
    })
    .map(|(str_command, coord1, coord2)| {
        Instruction {
            command: match str_command {
                "turn on" => Command::TurnOn,
                "turn off" => Command::TurnOff,
                "toggle" => Command::Toggle,
                _ => panic!("Instrução não reconhecida!")
            },
            initial_coordinates: (coord1.0.parse().unwrap(), coord1.1.parse().unwrap()),
            final_coordinates: (coord2.0.parse().unwrap(), coord2.1.parse().unwrap()),
        }
    }).collect()
}

fn execute_instruction(instruction: &Instruction, lights: &mut LightsArray) {
    let Instruction { 
        command, 
        initial_coordinates, 
        final_coordinates 
    } = instruction;

    for i in initial_coordinates.0..=final_coordinates.0 {
        if initial_coordinates.1 == final_coordinates.1 {
            let j = initial_coordinates.1;
            switch_light(lights, command, i, j);
        } else {
            for j in initial_coordinates.1..=final_coordinates.1 {
                switch_light(lights, command, i, j);
            }
        }
    }
}

fn switch_light(lights: &mut LightsArray, command: &Command, x: usize, y: usize ) {
    lights[x][y] = match command {
        Command::TurnOn => true,
        Command::TurnOff => false,
        Command::Toggle => !lights[x][y]
    }
}