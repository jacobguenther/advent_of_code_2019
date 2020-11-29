// File: day_6.rs
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
use std::collections::HashMap;

type OrbitMap = HashMap<String, Vec<String>>;

pub struct Challenge {}
impl ChallengeT for Challenge {
	type Output1 = u64;
	type Output2 = usize;

	fn day() -> i32 {
		6
	}
	fn part_1() -> Self::Output1 {
		let mut lines = get_lines_from_file("day_6_1.txt");
		let orbit_map = Challenge::build_orbit_map(&mut lines);
		Challenge::count_indirect_orbits("COM", 0, &orbit_map)
	}
	fn part_2() -> Self::Output2 {
		let mut lines = get_lines_from_file("day_6_1.txt");
		OrbitTree::new(&mut lines)
			.distance_to_santa()
	}

}
impl Challenge {
	// **** Part 1 Helpers **** \\
	fn parse_line(line: &str) -> (String, String) {
		let parts: Vec<&str> = line.split(")")
			.collect();
		assert_eq!(2, parts.len());
		(parts[1].to_owned(), parts[0].to_owned())
	}
	fn build_orbit_map(lines: &mut Vec<String>) -> OrbitMap {
		let mut orbit_map: OrbitMap = HashMap::new();
		for line in lines.drain(..) {
			let (satellite, base) = Challenge::parse_line(&line);
			match orbit_map.get_mut(&base) {
				Some(satellites) => {
					satellites.push(satellite);
				}
				None => {
					let mut satellites = Vec::new();
					satellites.push(satellite);
					orbit_map.insert(base, satellites);
				}
			};
		}
		orbit_map
	}
	fn count_indirect_orbits(base: &str, depth: u64, orbit_map: &OrbitMap) -> u64 {
		match orbit_map.get(base) {
			Some(satellites) => {
				satellites.iter()
					.map(|satellite| {
						Challenge::count_indirect_orbits(&satellite, depth+1, orbit_map)
					})
					.sum::<u64>()
				+ depth
			}
			None => depth
		}
	}
}

// a wrapper for an adjacency list
struct OrbitTree {
	start: String,
	adjacency_table: OrbitMap,
}
impl OrbitTree {
	fn new(lines: &mut Vec<String>) -> Self {
		let (start, adjacency_table) = {
			let satellite_base: HashMap<String, String> = lines
				.iter()
				.map(|line| Challenge::parse_line(&line))
				.collect();

			let start = satellite_base.get("YOU").unwrap().to_owned();

			let adjacency_table = Challenge::build_orbit_map(lines)
				.drain()
				.map(|(base, satellites)| {
					let mut adjacency = satellites;
					if let Some(parent) = satellite_base.get(&base) {
						adjacency.push(parent.to_owned());
					}
					(base, adjacency)
				})
				.collect();
			(start, adjacency_table)
		};

		OrbitTree {
			start: start,
			adjacency_table: adjacency_table,
		}
	}

	fn distance_to_santa(&self) -> usize {
		let mut path = Vec::with_capacity(self.adjacency_table.len());
		self.distance_to_santa_recursive(&self.start, &mut path);
		path.len() - 1
	}
	#[inline]
	fn distance_to_santa_recursive(&self, current: &str, path: &mut Vec<String>) -> bool
	{
		path.push(current.to_owned());

		if let Some(adjacent) = self.adjacency_table.get(current) {
			let original_length = path.len();
			for node in adjacent.iter() {
				if path.contains(node) {
					continue;
				} else if node == "SAN" {
					return true;
				}
				if self.distance_to_santa_recursive(node, path) {
					return true;
				} else {
					path.truncate(original_length);
				}
			}
		}

		path.pop();
		false
	}
}

#[cfg(test)]
mod tests {
	use crate::common::*;
	use super::Challenge;
	use super::OrbitTree;

	use crate::test::Bencher;

	#[test]
	fn part_1_pre_test() {
		let mut lines = get_lines_from_file("day_6_1_example.txt");
		let orbit_map = Challenge::build_orbit_map(&mut lines);
		let total_orbits = Challenge::count_indirect_orbits("COM", 0, &orbit_map);
		assert_eq!(42, total_orbits);
	}
	#[test]
	fn part_1_test() {
		assert_eq!(278744, Challenge::part_1());
	}

	#[test]
	fn part_2_pre_test() {
		let mut lines = get_lines_from_file("day_6_2_example.txt");
		let distance = OrbitTree::new(&mut lines).distance_to_santa();
		assert_eq!(4, distance);
	}
	#[test]
	fn part_2_test() {
		assert_eq!(475, Challenge::part_2());
	}

	#[bench]
	fn part_1_bench_prep_map(b: &mut Bencher) {
		let original = get_lines_from_file("day_6_1.txt");
		b.iter(|| {
			let mut lines = original.clone();
			Challenge::build_orbit_map(&mut lines)
		});
	}
	#[bench]
	fn part_1_bench_count_orbits(b: &mut Bencher) {
		let mut lines = get_lines_from_file("day_6_1.txt");
		let orbit_map = Challenge::build_orbit_map(&mut lines);
		b.iter(|| Challenge::count_indirect_orbits("COM", 0, &orbit_map));
	}
	#[bench]
	fn part_2_bench_prep_tree(b: &mut Bencher) {
		let original = get_lines_from_file("day_6_1.txt");
		b.iter(|| {
			let mut lines = original.clone();
			OrbitTree::new(&mut lines);
		});
	}
	#[bench]
	fn part_2_bench_distance_to_santa(b: &mut Bencher) {
		let mut lines = get_lines_from_file("day_6_1.txt");
		let map = OrbitTree::new(&mut lines);
		b.iter(|| map.distance_to_santa());
	}
}
