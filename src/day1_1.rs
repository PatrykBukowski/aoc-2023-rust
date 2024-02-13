use std::{fs::File, io::Read};

pub fn run() {
    let mut file = File::open("src/day1_1.txt").unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    let content_vector = contents.split("\n");

    let mut new_vec: Vec<u32> = Vec::new();

    for element in content_vector {
        let mut preresult = String::new();

        for current_char in element.chars() {
            if current_char.is_digit(10) {
                preresult.push(current_char);
            }
        }
        let result = preresult.to_owned().chars().next().unwrap().to_string()
            + &preresult.to_owned().chars().last().unwrap().to_string();
        new_vec.push(result.parse().unwrap());
    }

    let res: u32 = new_vec.iter().sum();

    println!("{res}");
}
