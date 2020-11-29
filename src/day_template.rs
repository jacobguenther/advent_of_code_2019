// File: day_template.rs
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

pub struct Challenge {}
impl ChallengeT for Challenge {
	type Output1 = i32;
	type Output2 = i32;

	fn day() -> i32 {
		0
	}
	fn part_1() -> Self::Output1 {
		0
	}
	fn part_2() -> Self::Output2 {
		0
	}
}

#[cfg(test)]
mod tests {
	use super::Challenge;
	use crate::common::*;
	use crate::test::Bencher;

	#[test]
	fn part_1_test() {
		let res = Challenge::part_1();
		assert_eq!(res, 0);
	}
	#[test]
	fn part_2_test() {
		let res = Challenge::part_2();
		assert_eq!(res, 0);
	}

	#[bench]
	fn part_1_bench(b: &mut Bencher) {
		b.iter(|| 1+1);
	}

	#[bench]
	fn part_2_bench(b: &mut Bencher) {
		b.iter(|| 1+1);
	}
}
