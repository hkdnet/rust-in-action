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
        let new_elem = Self::allocate_in_heap(new_cap);
        let old_elements = std::mem::replace(&mut self.elements, new_elem);
        for (i, elem) in old_elements.into_vec().into_iter().enumerate() {
            self.elements[i] = elem;
        }
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            elements: &self.elements,
            len: self.len(),
            pos: 0,
        }
    }
}

impl<'vec, T: Default> IntoIterator for &'vec ToyVec<T> {
    type Item = &'vec T;
    type IntoIter = Iter<'vec, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct Iter<'vec, T> {
    elements: &'vec Box<[T]>,
    len: usize,
    pos: usize, // the next element's index
}

impl<'vec, T> Iterator for Iter<'vec, T> {
    type Item = &'vec T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.len {
            None
        } else {
            let e = &self.elements[self.pos];
            self.pos += 1;
            Some(e)
        }
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

    #[test]
    fn test_for() {
        let mut vec = ToyVec::<u8>::new();
        vec.push(0);
        vec.push(1);
        vec.push(2);

        let mut expected = Vec::new();
        for &e in vec.iter() {
            expected.push(e);
        }
        assert_eq!(expected, vec![0, 1, 2]);
    }
}
