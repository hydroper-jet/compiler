use std::rc::Rc;

/// A shared mutable array of `T`.
#[derive(Clone)]
pub struct SharedArray<T>(Rc<Vec<T>>);

impl<T> PartialEq for SharedArray<T> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl<T> Eq for SharedArray<T> {}

impl<T> SharedArray<T> {
    pub fn new() -> Self {
        Self(Rc::new(vec![]))
    }

    pub fn get(&self, index: usize) -> Option<T> where T: Clone {
        self.0.get(index).map(|v| v.clone())
    }

    pub fn set(&mut self, index: usize, value: T) where T: Clone {
        Rc::get_mut(&mut self.0).unwrap()[index] = value.clone();
    }

    pub fn remove(&mut self, index: usize) {
        Rc::get_mut(&mut self.0).unwrap().remove(index);
    }

    pub fn contains(&self, value: T) -> bool {}

    pub fn push(&mut self, value: T) where T: Clone {
        Rc::get_mut(&mut self.0).unwrap().push(value.clone());
    }
}

impl<T> IntoIterator for SharedArray<T> {
    type Item = T;
    type IntoIter = SharedArrayIterator<T>;
    fn into_iter(self) -> Self::IntoIter {
        SharedArrayIterator {
            array: &self,
            index: 0,
        }
    }
}

pub struct SharedArrayIterator<'a, T> {
    array: &'a SharedArray<T>,
    index: usize,
}

impl<'a, T> Iterator for SharedArrayIterator<'a, T> where T: Clone {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        let v = self.array.get(self.index);
        if v.is_some() {
            self.index += 1;
            v
        } else {
            None
        }
    }
}

impl<T> FromIterator<T> for SharedArray<T> where T: Clone {
    fn from_iter<T2: IntoIterator<Item = T>>(iter: T2) -> Self {
        let mut r = Self::new();
        for v in iter {
            r.push(v.clone());
        }
        r
    }
}