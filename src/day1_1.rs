use std::{fs::File, io::Read};

pub fn run() {
    let contents = read_content("src/day1_2.txt");
    let mut new_vec: Vec<u32> = Vec::new();
    for element in contents {
        if element.len() == 0 {
            continue;
        }
        new_vec.push(get_value_from_string(element));
    }
    let res: u32 = new_vec.iter().sum();
    println!("{res}");
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

fn get_value_from_string(value: String) -> u32 {
    let mut values = Vec::new();
    for current_char in value.chars() {
        if current_char.is_digit(10) {
            values.push(current_char.to_string());
        }
    }
    let mut result: Vec<String> = Vec::new();
    result.push(values.first().unwrap_or(&String::new()).to_string());
    result.push(values.last().unwrap_or(&String::new()).to_string());
    let result = result.concat();
    result.parse().unwrap_or(0)
}
