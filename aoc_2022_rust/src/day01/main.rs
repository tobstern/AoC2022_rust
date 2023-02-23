// Advent of Code solution //

// bring into scope: use ...
use std::fs

fn read_txt(String: day) -> String {
    // read in puzzle input: dayXX.txt
    let file_path = fs::read_to_string("../puzzle_inputs/day{:?}.txt", day).expect("Could not open the text-file")
}

fn main() {
    let day = String::from("01");

    // read in the text-file
    let txt = read_txt(day);
}
