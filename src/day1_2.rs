use std::{fs::File, io::Read};

pub fn run() {
    let contents = read_content("src/day1_4.txt");
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

struct ArrRefS<'a> {
    v: &'a str,
    r: &'a str,
}

fn get_value_from_string(value: String) -> u32 {
    let array_ref = [
        ArrRefS { v: "0", r: "0" },
        ArrRefS { v: "zero", r: "0" },
        ArrRefS { v: "1", r: "1" },
        ArrRefS { v: "one", r: "1" },
        ArrRefS { v: "two", r: "2" },
        ArrRefS { v: "2", r: "2" },
        ArrRefS { v: "3", r: "3" },
        ArrRefS { v: "three", r: "3" },
        ArrRefS { v: "4", r: "4" },
        ArrRefS { v: "four", r: "4" },
        ArrRefS { v: "5", r: "5" },
        ArrRefS { v: "five", r: "5" },
        ArrRefS { v: "6", r: "6" },
        ArrRefS { v: "six", r: "6" },
        ArrRefS { v: "7", r: "7" },
        ArrRefS { v: "seven", r: "7" },
        ArrRefS { v: "8", r: "8" },
        ArrRefS { v: "eight", r: "8" },
        ArrRefS { v: "9", r: "9" },
        ArrRefS { v: "nine", r: "9" },
    ];

    let mut ee: Vec<(usize, &str)> = Vec::new();
    for el in array_ref {
        match value.find(el.v) {
            Some(val) => ee.push((val, el.r)),
            None => {}
        }
        match value.rfind(el.v) {
            Some(val) => ee.push((val, el.r)),
            None => {}
        }
    }
    ee.sort_by(|a, b| a.0.cmp(&b.0));
    let mut result: Vec<String> = Vec::new();
    result.push(ee.first().unwrap_or(&(0, "")).1.to_string());
    result.push(ee.last().unwrap_or(&(0, "")).1.to_string());
    let result = result.concat();
    result.parse().unwrap_or(0)
}
