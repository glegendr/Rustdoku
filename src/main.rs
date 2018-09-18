use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
extern crate libdoku;
use libdoku::*;

fn read_file() -> Vec<Option<u8>> {
	if !Path::new("sudoku.txt").exists() {
		panic!("This is a terrible mistake ! You don't create sudoku.txt");
	}
	let mut f = File::open("sudoku.txt").expect("file not found");
	let mut contents = String::new();
	f.read_to_string(&mut contents).expect("something went wrong reading the file");
	let mut vec: Vec<Option<u8>> = Vec::new();
	for (_, c) in contents.chars().enumerate() {
		if c <= '9' && c > '0' {
			vec.push(Some(c as u8 - 48));
		} else if c == '.' {
			vec.push(None);
		} else if c != '\n' {
			panic!("Your sudoku is weird, i found '{}'", c);
		}
	}
	vec
}

fn main() {
	let grill = read_file();
	match Rustdoku::new(grill) {
		Ok(mut solver) => {
			match solver.solve() {
				Ok(rustdoku) => {
					println!("{}", rustdoku);
				},
					Err(er) => println!("Error: Grill not well formated ({:?})", er),
			}
		},
			Err(er) => println!("Error: Grill not well formated ({:?})", er),
	};
}
