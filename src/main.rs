#![feature(iter_array_chunks)]

mod clip;
mod winnow;

fn main() {}

fn foo<const N: usize>(arr: [i32; N]) -> i64 {
	let mut accum = 0i64;

	for i in arr {
		accum += (i as i64);
	}

	accum
}

fn bar<const N: usize>(arr: &[i32]) -> i64 {
	let mut accum = 0i64;

	let mut a = 0i32;

	for i in arr {
		accum += (*i as i64);
	}

	accum
}
