pub struct PracticalMapIterator<I, P> {
	backend: I,
	projector: P,
}

pub trait PracticalMap<T, U, I, P>
where
	I: Iterator<Item = T>,
	P: FnMut(T) -> U,
{
	fn my_map(self, projector: P) -> PracticalMapIterator<I, P>;
}

impl<T, U, I, P> PracticalMap<T, U, I, P> for I
where
	I: Iterator<Item = T>,
	P: FnMut(T) -> U,
{
	fn my_map(self, projector: P) -> PracticalMapIterator<I, P> {
		PracticalMapIterator {
			backend: self,
			projector,
		}
	}
}

impl<T, U, I, P> Iterator for PracticalMapIterator<I, P>
where
	I: Iterator<Item = T>,
	P: FnMut(T) -> U,
{
	type Item = U;

	fn next(&mut self) -> Option<Self::Item> {
		match self.backend.next() {
			None => Option::<Self::Item>::None,
			Some(s) => Some((self.projector)(s)),
		}
	}
}
