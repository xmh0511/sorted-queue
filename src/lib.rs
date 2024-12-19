use std::collections::LinkedList;

#[derive(Debug)]
pub enum Error {
    DuplicateAt(usize),
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
    pub fn push(&mut self, value: T) -> Result<(), Error> {
        let r = self
            .0
            .iter()
            .enumerate()
            .find(|(_index, ele)| value <= **ele)
            .map(
                |(index, ele)| {
                    if *ele == value {
                        Err(index)
                    } else {
                        Ok(index)
                    }
                },
            );
        match r {
            None => self.0.push_back(value),
            Some(Ok(index)) => self.insert(index, value),
            Some(Err(index)) => {
                // panic!("This queue should be strict total order, however, the pushed value would equal to the element at index {index}");
                return Err(Error::DuplicateAt(index));
            }
        }
        Ok(())
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
