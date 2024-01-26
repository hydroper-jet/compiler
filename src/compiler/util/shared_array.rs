use std::rc::Rc;

/// A shared mutable array of `T` managed by reference counting.
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

    pub fn contains(&self, value: T) -> bool where T: Clone + PartialEq {
        for value_2 in self.iter() {
            if value_2 == value {
                return true;
            }
        }
        false
    }

    pub fn index_of(&self, value: T) -> Option<usize> where T: PartialEq {
        for i in 0..self.length() {
            let value_2 = self.0.get(i).unwrap();
            if &value == value_2 {
                return Some(i);
            }
        }
        None
    }

    pub fn length(&self) -> usize {
        self.0.len()
    }

    pub fn push(&mut self, value: T) where T: Clone {
        Rc::get_mut(&mut self.0).unwrap().push(value.clone());
    }

    pub fn iter(&self) -> SharedArrayIterator<T> {
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