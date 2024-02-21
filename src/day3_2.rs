use std::{fmt::Error, fs::File, io::Read};
#[derive(Debug)]
struct Position {
    x: usize,
    y: usize,
}
#[derive(Debug)]
struct Num {
    value: String,
    position: Vec<Position>,
}

#[derive(Debug)]
struct Gear {
    x: usize,
    y: usize,
}

pub fn run(filename: &str) {
    let content = get_content_from_file(filename);
    let content = parse_content_to_vec_of_vec(content);

    let mut number: Num = Num {
        value: String::new(),
        position: Vec::new(),
    };

    let mut number_vec: Vec<Num> = Vec::new();
    let mut gear_vec: Vec<Gear> = Vec::new();

    for (index_mother, val_mother) in content.iter().enumerate() {
        for (index_son, val_son) in val_mother.iter().enumerate() {
            if val_son.is_numeric() {
                number.value = number.value + &val_son.to_string();
                number.position.push(Position {
                    x: index_son,
                    y: index_mother,
                });
            } else {
                if *val_son == '*' {
                    gear_vec.push(Gear {
                        x: index_son,
                        y: index_mother,
                    });
                }
                number_vec.push(number);
                number = Num {
                    value: String::new(),
                    position: Vec::new(),
                };
                continue;
            }
        }
    }

    let number_vec = number_vec
        .into_iter()
        .filter(|val| val.value.len() > 0)
        .collect::<Vec<Num>>();

    println!("{}", check_values(&number_vec, gear_vec));
}

fn get_content_from_file(filename: &str) -> String {
    let mut file = File::open(filename).expect("File not exists");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("File is empty");

    content
}

fn parse_content_to_vec_of_vec(content: String) -> Vec<Vec<char>> {
    let mut mother: Vec<Vec<char>> = Vec::new();

    content.lines().for_each(|line| {
        let mut son: Vec<char> = Vec::new();
        line.chars().for_each(|cha| {
            son.push(cha);
        });
        mother.push(son);
    });

    mother
}

fn check_values(numbers: &Vec<Num>, gears: Vec<Gear>) -> u32 {
    let mut res: Vec<u32> = Vec::new();
    for gear in gears {
        let numbers: Vec<&Num> = numbers
            .into_iter()
            .filter(|val| {
                let val = val
                    .position
                    .iter()
                    .any(|val2| check_current_value(val2.x, val2.y, gear.x, gear.y));
                val
            })
            .collect();

        if numbers.len() == 2 {
            let val1: u32 = numbers[0].value.parse().unwrap();
            let val2: u32 = numbers[1].value.parse().unwrap();
            res.push(val1 * val2);
        }
    }
    res.iter().sum()
}

fn check_current_value(
    value_a_x: usize,
    value_a_y: usize,
    value_b_x: usize,
    value_b_y: usize,
) -> bool {
    let ax: isize = value_a_x.try_into().unwrap();
    let ay: isize = value_a_y.try_into().unwrap();
    let bx: isize = value_b_x.try_into().unwrap();
    let by: isize = value_b_y.try_into().unwrap();

    (ax - 1 == bx && ay - 1 == by)
        || (ax - 1 == bx && ay == by)
        || (ax - 1 == bx && ay + 1 == by)
        || (ax == bx && ay + 1 == by)
        || (ax + 1 == bx && ay + 1 == by)
        || (ax + 1 == bx && ay == by)
        || (ax + 1 == bx && ay - 1 == by)
        || (ax == bx && ay - 1 == by)
}
