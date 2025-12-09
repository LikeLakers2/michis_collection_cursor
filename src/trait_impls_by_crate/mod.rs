macro_rules! forward_ref {
	() => {
		fn len(&self) -> usize {
			self.len()
		}

		fn get_item(&self, index: usize) -> Option<&Self::Item> {
			self.get(index)
		}
	};
}

macro_rules! forward_mut {
	(check_len_on_remove = $check_len:tt) => {
		forward_mut!(__inner, main);
		forward_mut!(__inner, remove, check_len = $check_len);
	};
	(__inner, main) => {
		fn get_item_mut(&mut self, index: usize) -> Option<&mut Self::Item> {
			self.get_mut(index)
		}

		fn set_item(&mut self, index: usize, item: Self::Item) {
			self.insert(index, item);
		}

		fn clear(&mut self) {
			self.clear();
		}
	};
	(__inner, remove, check_len = true) => {
		fn remove_item(&mut self, index: usize) -> Option<Self::Item> {
			(index < self.len()).then(|| self.remove(index))
		}
	};
	(__inner, remove, check_len = false) => {
		fn remove_item(&mut self, index: usize) -> Option<Self::Item> {
			self.remove(index)
		}
	};
}

/// Checks that the `forward_ref!()` and `forward_mut!()` macros provide consistent results.
#[cfg(test)]
mod forward_macro_consistency_tests {
	extern crate alloc;
	use core::ops::{Deref, DerefMut};

	use alloc::vec::Vec;

	use crate::{IndexableCollection, IndexableCollectionMut};

	/// Returns `None` on a bad remove
	struct TestVec(Vec<i32>);

	impl TestVec {
		pub fn len(&self) -> usize {
			self.0.len()
		}

		pub fn get(&self, index: usize) -> Option<&i32> {
			self.0.get(index)
		}

		pub fn get_mut(&mut self, index: usize) -> Option<&mut i32> {
			self.0.get_mut(index)
		}

		pub fn insert(&mut self, index: usize, item: i32) {
			self.0.insert(index, item);
		}

		pub fn remove(&mut self, index: usize) -> Option<i32> {
			(index < self.len()).then(|| self.0.remove(index))
		}

		pub fn clear(&mut self) {
			self.0.clear();
		}
	}

	impl IndexableCollection for TestVec {
		type Item = i32;
		forward_ref!();
	}

	impl IndexableCollectionMut for TestVec {
		forward_mut!(check_len_on_remove = false);
	}

	/// Wrapper around `TestVec` to make it panic on a bad remove, so we can test the macro's "check
	/// len" functionality
	struct PanicVec(TestVec);

	impl PanicVec {
		pub fn remove(&mut self, index: usize) -> i32 {
			self.0.remove(index).expect("removal index higher than len")
		}
	}

	#[allow(
		dead_code,
		reason = "These are here solely to prevent infinite recursion warnings on the macro, due \
		to trait methods taking priority over a deref. In a real scenario, either a deref wouldn't \
		happen, or the user would handle it themselves."
	)]
	impl PanicVec {
		pub fn len(&self) -> usize {
			self.deref().len()
		}

		pub fn clear(&mut self) {
			self.deref_mut().clear();
		}
	}

	impl Deref for PanicVec {
		type Target = TestVec;
		fn deref(&self) -> &Self::Target {
			&self.0
		}
	}

	impl DerefMut for PanicVec {
		fn deref_mut(&mut self) -> &mut Self::Target {
			&mut self.0
		}
	}

	impl IndexableCollection for PanicVec {
		type Item = i32;
		forward_ref!();
	}

	impl IndexableCollectionMut for PanicVec {
		forward_mut!(check_len_on_remove = true);
	}

	/// Ensure that the length reported by the trait is the same as the length reported by the inner
	/// collection.
	#[test]
	fn len_consistency() {
		let inputs: Vec<Vec<i32>> = [[].into(), [0].into(), [1, 2].into(), [3, 4, 5].into()].into();
		inputs.into_iter().for_each(|input| {
			let collection = TestVec(input);
			let collection_defined_len = collection.len();
			let trait_defined_len = IndexableCollection::len(&collection);

			assert_eq!(trait_defined_len, collection_defined_len);
		});
	}

	/// Ensure that we receive the same items from the inner collection as from the trait.
	#[test]
	fn get_item_consistency() {
		let input = Vec::from([1, 2, 3]);

		let collection = TestVec(Vec::from_iter(input.clone()));
		input.iter().enumerate().for_each(|(index, _)| {
			let collection_defined_get = collection.get(index);
			let trait_defined_get = IndexableCollection::get_item(&collection, index);

			assert_eq!(trait_defined_get, collection_defined_get);
		});

		// Also test that we get the same item if we go past the end
		let collection_get_after_end = collection.get(input.len());
		let trait_get_after_end = IndexableCollection::get_item(&collection, input.len());
		assert_eq!(trait_get_after_end, collection_get_after_end);
	}

	#[test]
	fn get_item_mut_consistency() {
		let mut input = Vec::from([1, 2, 3]);

		let mut collection = TestVec(Vec::from_iter(input.clone()));
		input.iter_mut().enumerate().for_each(|(index, _)| {
			let collection_defined_get = collection.get_mut(index).cloned();
			let trait_defined_get =
				IndexableCollectionMut::get_item_mut(&mut collection, index).cloned();

			assert_eq!(trait_defined_get, collection_defined_get);
		});

		// Also test that we get the same item if we go past the end
		let collection_get_after_end = collection.get_mut(input.len()).cloned();
		let trait_get_after_end =
			IndexableCollectionMut::get_item_mut(&mut collection, input.len()).cloned();
		assert_eq!(trait_get_after_end, collection_get_after_end);
	}
}

#[cfg(feature = "alloc")]
mod alloc;

#[cfg(feature = "arrayvec")]
mod arrayvec;

#[cfg(feature = "smallvec")]
mod smallvec;

#[cfg(feature = "tinyvec")]
mod tinyvec;
