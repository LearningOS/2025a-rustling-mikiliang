/*
	heap
	This question requires you to implement a binary heap function
*/


use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default,
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
        // 将新元素添加到数组末尾
        self.items.push(value);
        self.count += 1;

        // 向上调整堆，维持堆的性质
        self.heapify_up(self.count);
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        // 如果没有子节点，返回当前索引
        if !self.children_present(idx) {
            return idx;
        }

        let left = self.left_child_idx(idx);
        let right = self.right_child_idx(idx);

        // 只有左子节点的情况
        if right > self.count {
            return left;
        }

        // 有左右两个子节点，根据比较器选择合适的子节点
        // 这里的"smallest"是相对于比较器而言的
        if (self.comparator)(&self.items[left], &self.items[right]) {
            left
        } else {
            right
        }
    }
    // 向上调整堆（用于插入操作）
    fn heapify_up(&mut self, mut idx: usize) {
        while idx > 1 {
            let parent = self.parent_idx(idx);

            // 如果当前节点不满足向上调整的条件，停止
            if !(self.comparator)(&self.items[idx], &self.items[parent]) {
                break;
            }

            // 交换当前节点和父节点
            self.items.swap(idx, parent);
            idx = parent;
        }
    }

    // 向下调整堆（用于删除操作）
    fn heapify_down(&mut self, mut idx: usize) {
        while self.children_present(idx) {
            let target_child = self.smallest_child_idx(idx);

            // 如果当前节点已经满足堆的性质，停止调整
            if !(self.comparator)(&self.items[target_child], &self.items[idx]) {
                break;
            }

            // 交换当前节点和目标子节点
            self.items.swap(idx, target_child);
            idx = target_child;
        }
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
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
    T: Default,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        // 取出堆顶元素（根节点，位于索引1）
        let root = std::mem::replace(&mut self.items[1], T::default());

        // 如果堆中只有一个元素
        if self.count == 1 {
            self.count = 0;
            self.items.pop(); // 移除最后一个元素
            return Some(root);
        }

        // 将最后一个元素移到根节点位置
        let last = self.items.pop().unwrap();
        self.items[1] = last;
        self.count -= 1;

        // 向下调整堆，维持堆的性质
        self.heapify_down(1);

        Some(root)
    }

}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
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