use arrayvec::ArrayVec;

use crate::{IndexableCollection, IndexableCollectionMut};

impl<T, const CAP: usize> IndexableCollection for ArrayVec<T, CAP> {
	type Item = T;

	fn len(&self) -> usize {
		self.len()
	}

	fn get_item(&self, index: usize) -> Option<&Self::Item> {
		self.get(index)
	}
}

impl<T, const CAP: usize> IndexableCollectionMut for ArrayVec<T, CAP> {
	fn get_item_mut(&mut self, index: usize) -> Option<&mut Self::Item> {
		self.get_mut(index)
	}

	fn set_item(&mut self, index: usize, item: Self::Item) {
		self.insert(index, item);
	}

	fn remove_item(&mut self, index: usize) -> Option<Self::Item> {
		(index < self.len()).then(|| self.remove(index))
	}

	fn clear(&mut self) {
		self.clear();
	}
}
