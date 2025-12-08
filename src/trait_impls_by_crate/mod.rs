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
	(check_len_on_remove = true) => {
		forward_mut!(__main);

		fn remove_item(&mut self, index: usize) -> Option<Self::Item> {
			(index < self.len()).then(|| self.remove(index))
		}
	};
	(check_len_on_remove = false) => {
		forward_mut!(__main);

		fn remove_item(&mut self, index: usize) -> Option<Self::Item> {
			self.remove(index)
		}
	};
	(__main) => {
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
}

#[cfg(feature = "alloc")]
mod alloc;

#[cfg(feature = "arrayvec")]
mod arrayvec;

#[cfg(feature = "smallvec")]
mod smallvec;

#[cfg(feature = "tinyvec")]
mod tinyvec;
