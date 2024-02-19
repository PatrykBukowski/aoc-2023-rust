use std::{fs::File, io::Read};

struct Game {
    blue: u16,
    red: u16,
    green: u16,
}

pub fn run(filename: &str) {
    let contents = read_content(filename);
    let mut content_vec: Vec<u32> = Vec::new();
    contents.iter().for_each(|value| {
        if value.len() == 0 {
            return;
        }
        content_vec.push(get_value_from_string(value));
    });
    let result: u32 = content_vec.iter().sum();
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

fn get_value_from_string(value: &String) -> u32 {
    let mut first_split = value.split(":");
    first_split.next().unwrap();
    let second_split = first_split.next().unwrap().trim().split(";");
    let mut result: Vec<Game> = Vec::new();
    second_split.for_each(|turn| {
        let mut ga = Game {
            blue: 0,
            red: 0,
            green: 0,
        };
        turn.trim().split(",").for_each(|v| {
            let mut g = v.trim().split(" ");
            let value: u16 = g.next().unwrap().parse().unwrap();
            match g.next().unwrap() {
                "green" => ga.green = value,
                "red" => ga.red = value,
                _ => ga.blue = value,
            }
        });
        result.push(ga);
    });

    let max_blue: u32 = result.iter().map(|a| a.blue).max().unwrap().into();
    let max_red: u32 = result.iter().map(|a| a.red).max().unwrap().into();
    let max_green: u32 = result.iter().map(|a| a.green).max().unwrap().into();

    max_blue * max_red * max_green
}
