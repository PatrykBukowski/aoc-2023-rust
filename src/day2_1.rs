use std::{fs::File, io::Read};

const RED_CUBES: u8 = 12;
const GREEN_CUBES: u8 = 13;
const BLUE_CUBES: u8 = 14;

pub fn run(filename: &str) {
    let contents = read_content(filename);
    let mut content_vec: Vec<u16> = Vec::new();
    contents.iter().for_each(|value| {
        if value.len() == 0 {
            return;
        }
        content_vec.push(get_value_from_string(value));
    });
    let result: u16 = content_vec.iter().sum();
    println!("{result}");
}

fn read_content(filename: &str) -> Vec<String> {
    let mut file = File::open(filename).expect("File not found");
    let mut contents = String::new();
    let mut result: Vec<String> = Vec::new();

    file.read_to_string(&mut contents).unwrap_or(0);
    contents
        .split('\n')
        .for_each(|a| result.push(a.to_string()));

    result
}

fn get_value_from_string(value: &String) -> u16 {
    let mut first_split = value.split(":");
    let game_string: u8 = first_split
        .next()
        .unwrap()
        .split(" ")
        .last()
        .unwrap()
        .parse()
        .unwrap();
    let second_split = first_split.next().unwrap().trim().split(";");
    let mut is_possible = true;
    second_split.for_each(|turn| {
        turn.trim().split(",").for_each(|v| {
            let mut g = v.trim().split(" ");
            let value: u8 = g.next().unwrap().parse().unwrap();
            match g.next().unwrap() {
                "green" => {
                    if value > GREEN_CUBES {
                        is_possible = false;
                    }
                }
                "red" => {
                    if value > RED_CUBES {
                        is_possible = false;
                    }
                }
                _ => {
                    if value > BLUE_CUBES {
                        is_possible = false
                    }
                }
            }
        });
    });
    if is_possible {
        return game_string.into();
    }
    0
}
