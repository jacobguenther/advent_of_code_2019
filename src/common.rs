// File: common.rs
// Author: Jacob Guenther
// Date: December 2020

/*
Copyright 2020 Jacob Guenther

Permission is hereby granted, free of charge, to any person obtaining a copy of
this software and associated documentation files (the "Software"), to deal in
the Software without restriction, including without limitation the rights to
use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software is furnished to do so,
subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/

use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::fmt::Display;

pub trait ChallengeT {
	type Output1: Display;
	type Output2: Display;

	fn print_result() {
		println!("{}", Self::result_string());
	}
	fn result_string() -> String {
		format!("Day {}\n  part 1: {}\n  part 2: {}", Self::day(), Self::part_1(), Self::part_2())
	}

	fn day() -> i32;
	fn part_1() -> Self::Output1;
	fn part_2() -> Self::Output2;
}

fn file_path(file_name: &str) -> String {
	format!("inputs/{}", file_name)
}
pub fn read_file(file_name: &str) -> String {
	fs::read_to_string(&file_path(file_name))
		.expect(
			&format!("Something went wrong reading the file: {}", file_name)
		)
}

pub fn get_lines_from_file(file_name: &str) -> Vec<String> {
	let file = File::open(&file_path(file_name))
		.expect(
			&format!("Something went wrong opening the file: {}", file_name)
		);
	BufReader::new(file)
		.lines()
		.enumerate()
		.map(|(index, line)| {
			match line {
				Ok(text) => text,
				Err(err) => panic!("Failed while reading the file {} at line {} with error: {}", file_name, index, err),
			}
		})
		.collect()
}

#[cfg(test)]
mod tests {
	use super::{
		file_path,
		read_file,
		get_lines_from_file,
	};

	#[test]
	fn file_path_test() {
		let path = file_path("test.txt");
		assert_eq!(&path, "inputs/test.txt");
	}
	#[test]
	fn read_file_test() {
		let contents = read_file("test.txt");
		assert_eq!(&contents, "Hello World!\nWe come in peace");
	}
	#[test]
	fn get_lines_from_file_test() {
		let lines = get_lines_from_file("test.txt");
		assert_eq!(lines.len(), 2);
		assert_eq!(lines[0], "Hello World!");
		assert_eq!(lines[1], "We come in peace");
	}
}