pub struct WinnowIterator<I, P>(I, P);

pub trait Winnow<I, P> {
	fn winnow(self, pred: P) -> WinnowIterator<I, P>;
}

impl<T, I: Iterator<Item = T>, P: Fn(&T) -> bool> Winnow<I, P> for I {
	fn winnow(self, pred: P) -> WinnowIterator<I, P> {
		WinnowIterator(self, pred)
	}
}

impl<T, I: Iterator<Item = T>, P: Fn(&T) -> bool> Iterator for WinnowIterator<I, P> {
	type Item = T;

	fn next(&mut self) -> Option<Self::Item> {
		for s in self.0.by_ref() {
			if (self.1)(&s) {
				return Some(s);
			} else {
				continue;
			}
		}
		None
	}
}

pub enum WinnowTypes {
	Even,
	Odd,
}

impl<I: Iterator<Item = i32>> Winnow<I, WinnowTypes> for I {
	fn winnow(self, pred: WinnowTypes) -> WinnowIterator<I, WinnowTypes> {
		WinnowIterator(self, pred)
	}
}

impl<I: Iterator<Item = i32>> Iterator for WinnowIterator<I, WinnowTypes> {
	type Item = i32;

	fn next(&mut self) -> Option<Self::Item> {
		todo!()
	}
}

#[cfg(test)]
mod tests {
	use super::{Winnow, WinnowIterator};
	use std::fmt::Debug;

	fn assert<T: PartialEq + Debug, E: Iterator<Item = T>, A: Iterator<Item = T>>(
		mut expected: E,
		mut actual: A,
	) {
		for e in expected.by_ref() {
			match actual.by_ref().next() {
				None => unreachable!(),
				Some(a) => assert_eq!(e, a),
			}
		}

		match actual.by_ref().next() {
			None => assert!(true),
			Some(_) => unreachable!(),
		}
	}
	#[test]
	fn closure_test() {
		let expected = (0..10).filter(|i| i & 1 == 0);
		let actual = (0..10).winnow(|i: &i32| i & 1 == 0);

		assert(expected, actual)
	}
}
