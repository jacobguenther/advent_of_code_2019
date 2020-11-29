// File: day_8.rs
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

use super::common::*;

const LAYER_WIDTH: usize = 25;
const LAYER_HEIGHT: usize = 6;
const LAYER_SIZE: usize = LAYER_WIDTH * LAYER_HEIGHT;

type Row = Vec<Pixel>;
type Layer = Vec<Row>;
type Layers = Vec<Layer>;

pub struct Challenge {}

impl ChallengeT for Challenge {
	type Output1 = u32;
	type Output2 = String;

	fn day() -> i32 {
		8
	}
	fn part_1() -> Self::Output1 {
		let count = read_file("day_8_1.txt")
			.chars()
			.enumerate()
			.fold(&mut Accumulator::default(), |acc, (i, c)| {
				if i%LAYER_SIZE == 0 {
					if acc.current_count.twos > acc.most_2s_count.twos {
						acc.most_2s_count = acc.current_count;
					}
					acc.current_count = Count::default();
				}
				match c {
					'0' => acc.current_count.zeros += 1,
					'1' => acc.current_count.ones += 1,
					'2' => acc.current_count.twos += 1,
					_ => (),
				}
				acc
			})
			.most_2s_count;

		count.ones * count.twos
	}
	fn part_2() -> Self::Output2 {
		let contents = read_file("day_8_1.txt");

		let layer_count = contents.chars().count()/LAYER_SIZE;
		let mut layers = create_layers(layer_count);

		for (i, c) in contents.chars().enumerate() {
			let (layer, row, col) = get_coord_from_index(i);
			layers[layer][row][col] = c.into();
		}

		let mut out_pixels = create_layer();
		for layer in layers.iter() {
			merge_layers(&mut out_pixels, layer);
		}

		let mut out = String::new();
		out.push_str("\n");
		for row in out_pixels {
			for pixel in row {
				match pixel {
					Pixel::White => out.push_str("#"),
					Pixel::Black | Pixel::Tranparent => out.push_str(" "),
				}
			}
			out.push_str("\n");
		}
		out
	}
}

// **** Part 1 Helpers **** \\
#[derive(Copy, Clone)]
struct Count {
	zeros: u32,
	ones: u32,
	twos: u32,
}
impl Default for Count {
	fn default() -> Count {
		Count {
			zeros: 0,
			ones: 0,
			twos: 0,
		}
	}
}
#[derive(Copy, Clone)]
struct Accumulator {
	most_2s_count: Count,
	current_count: Count,
}
impl Default for Accumulator {
	fn default() -> Accumulator {
		Accumulator {
			most_2s_count: Count::default(),
			current_count: Count::default(),
		}
	}
}


// **** Part 2 Helpers **** \\
#[derive(Copy, Clone)]
enum Pixel {
	Black,
	White,
	Tranparent,
}
impl Default for Pixel {
	fn default() -> Pixel {
		Pixel::Tranparent
	}
}
impl From<char> for Pixel {
    fn from(n: char) -> Pixel {
        match n {
			'0' => Pixel::Black,
			'1' => Pixel::White,
			'2' => Pixel::Tranparent,
			_ => panic!("Unexpected pixel value"),
		}
    }
}
fn get_coord_from_index(i: usize) -> (usize, usize, usize) {
	let layer = i / LAYER_SIZE;
	let row = (i / LAYER_WIDTH) % LAYER_HEIGHT;
	let col = i % LAYER_WIDTH;
	(layer, row, col)
}
fn create_layers(layer_count: usize) -> Layers {
	let mut layers = Layers::with_capacity(layer_count);
	for _ in 0..layer_count {
		let layer = create_layer();
		layers.push(layer);
	}
	layers
}
fn create_layer() -> Layer {
	let mut output = Layer::with_capacity(LAYER_HEIGHT);
	for _ in 0..LAYER_HEIGHT {
		let mut row = Row::with_capacity(LAYER_WIDTH);
		for _ in 0..LAYER_WIDTH {
			row.push(Pixel::default())
		}
		output.push(row);
	}
	output
}
fn merge_layers(output: &mut Layer, to_merge: &Layer) {
	for row_i in 0..LAYER_HEIGHT {
		for col_i in 0..LAYER_WIDTH {
			let out_p = &output[row_i][col_i];
			let merge_p = &to_merge[row_i][col_i];
			match (out_p, merge_p) {
				(Pixel::Tranparent, color) => output[row_i][col_i] = *color,
				_ => (),
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::Challenge;
	use crate::common::*;

	#[test]
	fn part_1_test() {
		let res = Challenge::part_1();
		assert_eq!(828, res);
	}
	#[test]
	fn part_2_test() {
		let res = Challenge::part_2();
		assert_eq!(res, "\n#### #    ###    ## #### \n   # #    #  #    # #    \n  #  #    ###     # ###  \n #   #    #  #    # #    \n#    #    #  # #  # #    \n#### #### ###   ##  #    \n");
	}
}
