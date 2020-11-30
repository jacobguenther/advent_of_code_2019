// File: day_10.rs
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
use std::collections::HashSet;
use gcd::Gcd;

type StepRatios = HashSet<Vec2<i32>>;
type AstroidMap = Vec<Vec<char>>;

pub struct Challenge {}
impl ChallengeT for Challenge {
	type Output1 = usize;
	type Output2 = usize;

	fn day() -> i32 {
		10
	}
	fn part_1() -> Self::Output1 {
		let lines = get_lines_from_file("day_10_1.txt");
		let map = prep_map(&lines);
		let best_position = find_best_position(&map);
		best_position.count
	}
	fn part_2() -> Self::Output2 {
		0
	}
}

#[derive(Copy, Clone, Debug)]
struct BestPosition {
	at: Vec2<usize>,
	count: usize,
}
impl BestPosition {
	fn new(x: usize, y: usize, count: usize) -> Self {
		Self {
			at: Vec2::new(x, y),
			count: count,
		}
	}
}

fn prep_map(lines: &Vec<String>) -> AstroidMap {
	lines.iter()
		.map(|line| line.chars().collect())
		.collect()
}
fn find_best_position(map: &AstroidMap) -> BestPosition {
	let mut maybe_best_pos: Option<BestPosition> = None;
	let ratios = setup_ratios(&map_size(map));
	for (y, row) in map.iter().enumerate() {
		for (x, c) in row.iter().enumerate() {
			if *c != '#' {
				continue;
			}
			let count = count_visible(&map, &ratios, &Vec2::new(x, y));
			if let Some(best_pos) = maybe_best_pos {
				if count > best_pos.count {
					maybe_best_pos = Some(BestPosition::new(x, y, count));
				}
			} else {
				maybe_best_pos = Some(BestPosition::new(x, y, count));
			}
		}
	}
	maybe_best_pos.unwrap()
}
fn count_visible(map: &AstroidMap, ratios: &StepRatios, start: &Vec2<usize>) -> usize {
	let mut count = 0;
	for ratio in ratios {
		let mut check_pos = step_ratio(&start.into(), &ratio);
		let map_size = map_size(map);
		while in_bounds(&check_pos, &map_size) {
			let c = map[check_pos.y as usize][check_pos.x as usize];
			if c == '#' {
				count += 1;
				break;
			} else {
				check_pos = step_ratio(&check_pos, &ratio);
			}
		}
	}
	count
}
fn setup_ratios(map_size: &Vec2<usize>) -> StepRatios {
	let mut set = StepRatios::new();
	set.insert(Vec2::new( 1, 0));
	set.insert(Vec2::new( 0, 1));
	set.insert(Vec2::new(-1, 0));
	set.insert(Vec2::new( 0,-1));

	for dy in 1..map_size.y {
		for dx in dy..map_size.x {
			if dx != 1 && dx == dy {
				continue;
			}
			let divior = dx.gcd(dy);
			let (x, y) = ((dx/divior) as i32, (dy/divior) as i32);
			if !set.contains(&Vec2::new(x, y)) {
				set.insert(Vec2::new( x, y));
				set.insert(Vec2::new(-x, y));
				set.insert(Vec2::new( x,-y));
				set.insert(Vec2::new(-x,-y));

				set.insert(Vec2::new( y, x));
				set.insert(Vec2::new(-y, x));
				set.insert(Vec2::new( y,-x));
				set.insert(Vec2::new(-y,-x));
			}
		}
	}
	set
}
fn map_size(map: &AstroidMap) -> Vec2<usize> {
	Vec2::new(map[0].len(), map.len())
}
fn step_ratio(start: &Vec2<i32>, step: &Vec2<i32>) -> Vec2<i32> {
	Vec2::new(start.x + step.x, start.y + step.y)
}
fn in_bounds(pos: &Vec2<i32>, size: &Vec2<usize>) -> bool {
	pos.x >= 0 && pos.y >= 0 && pos.x < size.x as i32 && pos.y < size.y as i32
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test::Bencher;

	fn part_1_example_helper(file_name: &str, expect: &str) {
		let lines = get_lines_from_file(file_name);
		let map = prep_map(&lines);
		let best_position = super::find_best_position(&map);
		assert_eq!(expect, &format!("{},{} = {}", best_position.at.x, best_position.at.y, best_position.count));
	}
	#[test]
	fn part_1_example_test_1() {
		part_1_example_helper("day_10_1_example_1.txt", "3,4 = 8");
	}
	#[test]
	fn part_1_example_test_2() {
		part_1_example_helper("day_10_1_example_2.txt", "6,3 = 41");
	}
	#[test]
	fn part_1_example_test_3() {
		part_1_example_helper("day_10_1_example_3.txt", "11,13 = 210");
	}

	#[test]
	fn part_1_test() {
		let res = Challenge::part_1();
		assert_eq!(res, 288);
	}
	#[test]
	fn part_2_test() {
		let res = Challenge::part_2();
		assert!(res < 821);
		assert_eq!(res, 0);
	}

	#[bench]
	fn get_lines(b: &mut Bencher) {
		b.iter(|| get_lines_from_file("day_10_1.txt"));
	}
	#[bench]
	fn part_1_bench_setup_ratios(b: &mut Bencher) {
		let lines = get_lines_from_file("day_10_1.txt");
		let map = prep_map(&lines);
		b.iter(|| setup_ratios(&map_size(&map)));
	}
}
