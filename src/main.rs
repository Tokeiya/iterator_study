#![feature(iter_array_chunks)]

mod clip;
mod winnow;

fn main() {
	let chunk = (0..9).array_chunks::<5>();

	for clip in chunk {
		println!("----------");
		for i in clip {
			println!("{}", i)
		}
	}
}
