// Advent of Code solution //
// bring into scope: use ...
use std::fs;

fn read_txt(day: String) -> String {
    // read in puzzle input: dayXX.txt
    let suffix: &str = ".txt";
    
    let file_path: String = "../puzzle_inputs/day".to_owned() + &day + &suffix;
    
    let text: String = fs::read_to_string(file_path)
        .expect("Could not open the text-file");

    return text
}

fn main() {
    let day = String::from("01");

    // read in the text-file
    let txt: String = read_txt(day);

    println!("{:?}", txt);
}
