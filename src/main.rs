use std::fs::File;
use std::io;
use std::io::Read;

fn main() {
    let goal = 19690720;
    let inputs = create_list(String::from("input.txt"));

    for noun in 0..=99 {
        for verb in 0..=99 {
            let inputs_clone = inputs.clone();
            let mut int_code_program = IntCodeProgram {
                inputs: inputs_clone,
            };
            if int_code_program.run_intcode_program(noun, verb) == goal {
                println!("Value is: {} ", 100 * noun + verb);

                return;
            }
        }
    }
}

pub struct IntCodeProgram {
    inputs: Vec<usize>,
}

impl IntCodeProgram {
    fn run_intcode_program(&mut self, noun: usize, verb: usize) -> usize {
        self.inputs[1] = noun;
        self.inputs[2] = verb;
        let mut increment: usize = 0;
        loop {
            let value_at_position_one = self.inputs[increment + 1];
            let value_at_position_two = self.inputs[increment + 2];
            let value_at_position_three = self.inputs[increment + 3];
            match self.inputs[increment] {
                1 => {
                    let sum_of_values =
                        self.inputs[value_at_position_one] + self.inputs[value_at_position_two];
                    self.inputs[value_at_position_three] = sum_of_values;
                }

                2 => {
                    let product_of_values =
                        self.inputs[value_at_position_one] * self.inputs[value_at_position_two];
                    self.inputs[value_at_position_three] = product_of_values;
                }

                99 => {
                    break;
                }

                error => panic!("There was a problem {:?}", error),
            }
            increment = increment + 4;
        }

        self.inputs[0]
    }
}

fn read_file(file_name: String) -> Result<String, io::Error> {
    let mut file = File::open(file_name)?;
    let mut s = String::new();

    file.read_to_string(&mut s)?;
    Ok(s)
}

fn create_list(file_name: String) -> Vec<usize> {
    let text_string = read_file(file_name);
    let list_of_numbers: Vec<usize>;
    let mut modules: Vec<usize> = Vec::new();

    list_of_numbers = text_string
        .unwrap()
        .split(",")
        .map(|s| s.trim().parse::<usize>().unwrap())
        .collect();

    for number in list_of_numbers {
        modules.push(number);
    }

    modules
}
