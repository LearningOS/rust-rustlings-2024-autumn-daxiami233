/*
	heap
	This question requires you to implement a binary heap function
*/

use std::cmp::Ord;
use std::default::Default;
use std::fmt::Display;
pub struct Heap<T>
where
    T: Default+Copy+Display,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default+Copy+Display,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        //TODO
        // println!("{}", self.count);
        if self.count == 0 {
            self.items[0] = value;
        } else {
            self.items.push(value);
        }
        println!("{}", self.items[0]);
        self.count += 1;
        let mut idx = self.len()-1;

        let mut parent_idx = self.parent_idx(idx);
        println!("first:{},{}", self.items[idx], self.items[parent_idx]);  
        while (self.comparator)(&self.items[idx], &self.items[parent_idx]) {
            let temp;
            temp = self.items[idx];
            self.items[idx] = self.items[parent_idx];
            self.items[parent_idx] = temp;
            idx = parent_idx;
            parent_idx = self.parent_idx(idx);
            println!("first:{}", self.items[0]);

        }
        


    }

    fn parent_idx(&self, idx: usize) -> usize {
        if idx == 0 {
            return 0;
        }
        (idx-1) / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2 + 1
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        //TODO
		0
    }
}

impl<T> Heap<T>
where
    T: Default + Ord + Copy+Display,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default + Copy+Display,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        //TODO
        println!("aaaaa{}", self.items[0]);
        if self.count == 0 {
            return None;
        }
        let ret = Some(self.items[0]);
        // println!("{}",<T as Into<T>>::into(ret.unwrap()));
        self.items[0] = self.items[self.count-1];
        self.count -= 1;
        self.items.pop();
        let len = self.count - 1;
        let mut idx = 0;
        let mut left_child_idx = self.left_child_idx(idx);
        let mut right_child_idx = self.right_child_idx(idx);
        loop {
            let mut found_smallest = true;
            if left_child_idx<len && (self.comparator)(&self.items[left_child_idx], &self.items[right_child_idx]) {
                self.items.swap(idx, left_child_idx);
                idx = left_child_idx;
                found_smallest = false;
                
            } else if right_child_idx<len && (self.comparator)(&self.items[right_child_idx], &self.items[left_child_idx]){
                self.items.swap(idx, right_child_idx);
                idx = right_child_idx;
                found_smallest = false;
            }
            if found_smallest {
                break;
            }
            left_child_idx = self.left_child_idx(idx);
            right_child_idx = self.right_child_idx(idx);
        }
		ret
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord + Copy + Display,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord + Copy + Display,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}