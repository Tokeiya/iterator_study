pub struct PracticalFilterIterator<I,P>(I,P);

pub trait PracticalFilter{
	fn filter(self,predicater:P)
}
