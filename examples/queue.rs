use sorted_queue::SortedQueue;

fn simple() {
    let mut queue = SortedQueue::new();
    queue.push(2).unwrap();
    queue.push(1).unwrap();
    queue.push(3).unwrap();
    println!("{:?}", queue);
    assert_eq!(queue.pop(), Some(1));
    assert_eq!(queue.pop(), Some(2));
    assert_eq!(queue.pop(), Some(3));
}
fn complex() {
    let mut queue = SortedQueue::new();
    queue.push(Packet(1, vec![1, 2, 3, 4])).unwrap();
    queue.push(Packet(0, vec![1, 2, 3])).unwrap();
    queue.push(Packet(1, vec![1, 2, 3])).unwrap();
    println!("{:?}", queue);
    assert_eq!(queue.pop(), Some(Packet(0, vec![1, 2, 3])));
    assert_eq!(queue.pop(), Some(Packet(1, vec![1, 2, 3])));
    assert_eq!(queue.pop(), Some(Packet(1, vec![1, 2, 3, 4])));
}

#[derive(Debug)]
struct Packet(i32, Vec<u8>);

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1.len() == other.1.len()
    }
}
impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.0.partial_cmp(&other.0) {
            Some(core::cmp::Ordering::Equal) => self.1.len().partial_cmp(&other.1.len()),
            ord => return ord,
        }
    }
}
fn main() {
    simple();
    let a = Packet(0, vec![1, 2, 3]);
    let b = Packet(1, vec![1, 2, 3]);
    let c = Packet(1, vec![1, 2, 3, 4]);
    assert!(a < b);
    assert!(b < c);
    complex();
}
