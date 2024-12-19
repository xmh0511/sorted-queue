use std::{collections::LinkedList, fmt::Debug};

#[derive(Debug)]
pub enum Error {
    DuplicateAt(usize),
}

enum ErrorInner<T> {
    DuplicateAt(usize),
    Replace(T),
}

#[derive(Debug)]
#[repr(transparent)]
pub struct SortedQueue<T: PartialOrd>(LinkedList<T>);

impl<T: PartialOrd> SortedQueue<T> {
    pub fn new() -> Self {
        SortedQueue(LinkedList::new())
    }
    fn insert(&mut self, index: usize, value: T) {
        let mut tail = self.0.split_off(index);
        self.0.push_back(value);
        self.0.append(&mut tail);
    }
    fn push_help(&mut self, mut value: T, overwrite: bool) -> Result<(), ErrorInner<T>> {
        let r = self
            .0
            .iter_mut()
            .enumerate()
            .rfind(|(_index, ele)| value >= **ele)
            .map(|(index, ele)| (index, ele));
        match r {
            None => self.0.push_front(value),
            Some((index, ele)) => {
                if *ele == value {
                    if overwrite {
                        std::mem::swap(ele, &mut value);
                        return Err(ErrorInner::Replace(value));
                    } else {
                        return Err(ErrorInner::DuplicateAt(index));
                    }
                } else {
                    if index + 1 == self.0.len() {
                        self.0.push_back(value);
                    } else {
                        self.insert(index + 1, value)
                    }
                }
            }
        }
        Ok(())
    }
    pub fn push(&mut self, value: T) -> Result<(), Error> {
        self.push_help(value, false).map_err(|e| {
            if let ErrorInner::DuplicateAt(index) = e {
                Error::DuplicateAt(index)
            } else {
                unreachable!()
            }
        })
    }
    pub fn push_overwrite(&mut self, value: T) -> Option<T> {
        match self.push_help(value, true) {
            Ok(_) => None,
            Err(ErrorInner::Replace(v)) => Some(v),
            _ => {
                unreachable!()
            }
        }
    }
    pub fn pop(&mut self) -> Option<T> {
        self.0.pop_front()
    }
    pub fn remove(&mut self, index: usize) -> Option<T> {
        let mut tail = self.0.split_off(index);
        let r = tail.pop_front();
        self.0.append(&mut tail);
        r
    }
    pub fn front(&self) -> Option<&T> {
        self.0.front()
    }
    pub fn back(&self) -> Option<&T> {
        self.0.back()
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn contains(&self, x: &T) -> bool {
        self.0.contains(x)
    }
    pub fn clear(&mut self) {
        self.0.clear();
    }
    pub fn iter(&self) -> std::collections::linked_list::Iter<'_, T> {
        self.0.iter()
    }
}

impl<T: PartialOrd> IntoIterator for SortedQueue<T> {
    type Item = <LinkedList<T> as IntoIterator>::Item;

    type IntoIter = <LinkedList<T> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[cfg(test)]
mod test {
    use crate::SortedQueue;

    #[test]
    fn sorted() {
        let mut queue = SortedQueue::new();
        queue.push(2).unwrap();
        queue.push(1).unwrap();
        queue.push(3).unwrap();
        assert_eq!(queue.pop(), Some(1));
        assert_eq!(queue.pop(), Some(2));
        assert_eq!(queue.pop(), Some(3));
    }
}
