use std::{fs::File, io::Read};

pub fn run(filename: &str) {
    let content = get_content_from_file(filename);
    let content = parse_content_to_vec_of_vec(content);

    let mut number: String = String::new();
    let mut number_vec: Vec<u32> = Vec::new();
    let mut is_around = false;

    for (index_mother, val_mother) in content.iter().enumerate() {
        for (index_son, val_son) in val_mother.iter().enumerate() {
            if val_son.is_numeric() {
                number = number + &val_son.to_string();
            } else {
                match number.parse() {
                    Ok(value) => {
                        if is_around {
                            number_vec.push(value)
                        }
                    }
                    Err(_) => {}
                }
                is_around = false;
                number = String::new();
                continue;
            }

            if index_mother > 0
                && index_son > 0
                && !content[index_mother - 1][index_son - 1].is_numeric()
                && content[index_mother - 1][index_son - 1] != '.'
            {
                is_around = true;
                continue;
            }
            if index_son > 0
                && !content[index_mother][index_son - 1].is_numeric()
                && content[index_mother][index_son - 1] != '.'
            {
                is_around = true;
                continue;
            }
            if index_mother < content.len() - 1
                && index_son > 0
                && !content[index_mother + 1][index_son - 1].is_numeric()
                && content[index_mother + 1][index_son - 1] != '.'
            {
                is_around = true;
                continue;
            }
            if index_mother < content.len() - 1
                && !content[index_mother + 1][index_son].is_numeric()
                && content[index_mother + 1][index_son] != '.'
            {
                is_around = true;
                continue;
            }
            if index_mother < content.len() - 1
                && index_son < content[index_mother].len() - 1
                && !content[index_mother + 1][index_son + 1].is_numeric()
                && content[index_mother + 1][index_son + 1] != '.'
            {
                is_around = true;
                continue;
            }
            if index_son < content[index_mother].len() - 1
                && !content[index_mother][index_son + 1].is_numeric()
                && content[index_mother][index_son + 1] != '.'
            {
                is_around = true;
                continue;
            }
            if index_mother > 0
                && index_son < content[index_mother].len() - 1
                && !content[index_mother - 1][index_son + 1].is_numeric()
                && content[index_mother - 1][index_son + 1] != '.'
            {
                is_around = true;
                continue;
            }
            if index_mother > 0
                && !content[index_mother - 1][index_son].is_numeric()
                && content[index_mother - 1][index_son] != '.'
            {
                is_around = true;
                continue;
            }
        }
    }

    let result: u32 = number_vec.iter().sum();

    println!("{result}");
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
