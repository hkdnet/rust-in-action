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

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            let elem = std::mem::replace(&mut self.elements[self.len], Default::default());
            Some(elem)
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn capacity(&self) -> usize {
        self.elements.len()
    }

    pub fn push(&mut self, elem: T) {
        if self.len() >= self.capacity() {
            self.grow();
        }
        self.elements[self.len] = elem;
        self.len += 1;
    }

    fn grow(&mut self) {
        let new_cap = if self.capacity() == 0 {
            1
        } else {
            self.capacity() * 2
        };
        let mut new_elem = Self::allocate_in_heap(new_cap);
        for i in 0..self.len() {
            new_elem[i] = std::mem::replace(&mut self.elements[i], Default::default());
        }
        self.elements = new_elem;
    }
}

#[cfg(test)]
mod tests {
    use crate::collection::ToyVec;

    #[test]
    fn test_get() {
        let mut vec = ToyVec::<u8>::new();
        assert_eq!(vec.get(0), None);
        assert_eq!(vec.get(1), None);
        assert_eq!(vec.get(2), None);
        vec.push(1);
        assert_eq!(vec.get(0), Some(&1));
        assert_eq!(vec.get(1), None);
        assert_eq!(vec.get(2), None);
        vec.push(2);
        assert_eq!(vec.get(0), Some(&1));
        assert_eq!(vec.get(1), Some(&2));
        assert_eq!(vec.get(2), None);
    }

    #[test]
    fn test_get_or() {
        let mut vec = ToyVec::<u8>::new();
        assert_eq!(vec.get(0), None);
        let a = 100;
        assert_eq!(vec.get_or(0, &a), &a);
        let b = 200;
        vec.push(b);
        assert_eq!(vec.get_or(0, &a), &b);
    }

    #[test]
    fn test_pop() {
        let mut vec = ToyVec::<u8>::new();
        assert_eq!(vec.pop(), None);
        let a = 100;
        let b = 200;
        vec.push(a);
        vec.push(b);
        assert_eq!(vec.len(), 2);
        assert_eq!(vec.pop(), Some(b));
        assert_eq!(vec.len(), 1);
    }
}
