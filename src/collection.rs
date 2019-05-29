pub struct ToyVec<T> {
    elements: Box<[T]>,
    len: usize,
}

impl<T: Default> ToyVec<T> {
    pub fn new() -> ToyVec<T> {
        Self::with_capacity(0)
    }
    pub fn with_capacity(cap: usize) -> ToyVec<T> {
        ToyVec {
            elements: Self::allocate_in_heap(cap),
            len: 0,
        }
    }
    fn allocate_in_heap(size: usize) -> Box<[T]> {
        std::iter::repeat_with(Default::default)
            .take(size)
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    pub fn get(self: &Self, idx: usize) -> Option<&T> {
        if idx >= self.len {
            None
        } else {
            Some(&self.elements[idx])
        }
    }

    pub fn get_or<'a>(&'a self, idx: usize, default: &'a T) -> &'a T {
        self.get(idx).unwrap_or(default)
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn capacity(&self) -> usize {
        self.elements.len()
    }

    pub fn push(&mut self, elem: T) {
        let cap = self.capacity();
        if self.len() >= cap {
            let new_cap = if cap == 0 { 1 } else { cap * 2 };
            // reallocation
            // TODO:
            //   let mut new_elem = Self::allocate_in_heap(new_cap);
            //   for i in 0..self.len() {
            //     new_elem[i] = self.elements[i];
            //   }
            let new_elem = Self::allocate_in_heap(new_cap);
            self.elements = new_elem;
        }
        self.elements[self.len] = elem;
        self.len += 1;
    }
}

#[cfg(test)]
mod tests {
    use crate::collection::ToyVec;

    #[test]
    fn test_get() {
        let mut vec = ToyVec::<u8>::new();
        assert_eq!(vec.get(0), None);
        vec.push(1);
        assert_eq!(vec.get(0), Some(&1));
    }

    #[test]
    fn test_get_or() {
        let vec = ToyVec::<u8>::new();
        assert_eq!(vec.get(0), None);
        let a = 100;
        assert_eq!(vec.get_or(0, &a), &a);
    }
}
