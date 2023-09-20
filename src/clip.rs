pub struct ClipIterator<const N: usize, I> {
	iterator: I,
}

pub trait Clip<I> {
	fn clip<const N: usize>(self) -> ClipIterator<N, I>;
}

impl<T: Default + Copy, I: Iterator<Item = T>> Clip<I> for I {
	fn clip<const N: usize>(self) -> ClipIterator<N, I> {
		ClipIterator { iterator: self }
	}
}

impl<const N: usize, T: Default + Copy, I: Iterator<Item = T>> Iterator for ClipIterator<N, I> {
	type Item = ([T; N], usize);

	fn next(&mut self) -> Option<Self::Item> {
		let mut result = [T::default(); N];
		let mut size = 0usize;

		for i in 0..N {
			if let Some(s) = self.iterator.next() {
				size += 1;
				result[i] = s;
			} else {
				break;
			}
		}

		if size == 0 {
			None
		} else {
			Some((result, size))
		}
	}
}

#[cfg(test)]
mod tests {
	use super::{Clip, ClipIterator};

	#[test]
	fn clip_test() {
		let c = (0..9).clip::<5>();

		for array in c {
			println!("---------------");
			for i in array.0 {
				println!("{}", i)
			}
		}
	}
}
