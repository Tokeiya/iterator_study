use std::marker::PhantomData;

pub struct ClipIterator<'a, const N: usize, T, I> {
	phantom: PhantomData<&'a T>,
	iterator: I,
	backend: [T; N],
}

pub trait Clip<'a, const N: usize, T, I> {
	fn clip(self) -> ClipIterator<'a, N, T, I>;
}

impl<'a, const N: usize, T, I: Iterator<Item = T>> Clip<'a, N, T, I> for I {
	fn clip(self) -> ClipIterator<'a, N, T, I> {
		todo!()
	}
}

impl<'a, const N: usize, T, I: Iterator<Item = T>> Iterator for ClipIterator<'a, N, T, I> {
	type Item = &'a [T];

	fn next(&mut self) -> Option<Self::Item> {
		todo!()
	}
}

#[cfg(test)]
mod tests {
	use super::{Clip, ClipIterator};
	#[test]
	fn clip_test() {
		let mut ite: ClipIterator<'_, 5, i32, std::ops::Range<i32>> = (0..10).clip();

		let actual = ite.next().unwrap();
		assert_eq!(actual.len(), 5);

		let actual = ite.next().unwrap();
		assert_eq!(actual.len(), 4);
	}
}
