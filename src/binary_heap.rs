pub trait BinaryHeap<T> {
    fn insert(&mut self, data: T, key: usize);

    fn extract(&mut self) -> Result<T, HeapError>;

    fn insert_extract(&mut self, data: T, key: usize) -> Result<T, HeapError>;

    fn search(&self, key: usize) -> Result<T, HeapError>;

    fn size(&self) -> usize;

    fn is_empty(&self) -> bool;
}

#[derive(Debug)]
pub enum HeapError {
    KeyNotFound,
    HeapEmpty,
}

struct BinaryHeapNode<T> {
    data: T,
    key: usize,
}

pub struct BinaryMinHeap<T> {
    heap: Vec<BinaryHeapNode<T>>,
    size: usize,
}

impl<T> BinaryMinHeap<T> {
    pub fn new(size: Option<usize>) -> Self {
        match size {
            None => BinaryMinHeap {
                heap: Vec::<BinaryHeapNode<T>>::with_capacity(100),
                size: 0,
            },
            Some(_) => BinaryMinHeap {
                heap: Vec::<BinaryHeapNode<T>>::with_capacity(size.unwrap()),
                size: 0,
            },
        }
    }

    fn up_heap(&mut self, node_index: usize) {
        let (mut parent_index, mut current_index) = (0usize, 0usize);
        if node_index != 0 {
            (parent_index, current_index) = ((node_index - 1) / 2, node_index);
        }

        loop {
            if self.heap[current_index].key < self.heap[parent_index].key {
                self.heap.swap(current_index, parent_index);
                if parent_index == 0 {
                    break;
                }
                current_index = parent_index;
                parent_index = (current_index - 1) / 2;
            } else {
                break;
            }
        }
    }

    fn down_heap(&mut self, node_index: usize) {
        let mut current_index: usize = node_index;
        let (mut left_index, mut right_index, mut smallest_index) = (
            (current_index * 2) + 1,
            (current_index * 2) + 2,
            current_index,
        );

        loop {
            if (left_index < self.size)
                && (self.heap[left_index].key < self.heap[smallest_index].key)
            {
                smallest_index = left_index;
            }
            if (right_index < self.size)
                && (self.heap[right_index].key < self.heap[smallest_index].key)
            {
                smallest_index = right_index;
            }

            if smallest_index != current_index {
                self.heap.swap(current_index, smallest_index);
                current_index = smallest_index;
                (left_index, right_index) = ((current_index * 2) + 1, (current_index * 2) + 2);
            } else {
                break;
            }
        }
    }
}

impl<T: Clone> BinaryHeap<T> for BinaryMinHeap<T> {
    fn insert(&mut self, data: T, key: usize) {
        self.heap.push(BinaryHeapNode { data, key });
        self.size += 1;
        self.up_heap(self.size - 1);
    }

    fn extract(&mut self) -> Result<T, HeapError> {
        if self.size == 0 {
            return Err(HeapError::HeapEmpty);
        }
        let data = self.heap[0].data.clone();

        self.size -= 1;
        if self.size > 0 {
            self.heap.swap(0, self.size);
            self.down_heap(0);
        }

        Ok(data)
    }

    fn insert_extract(&mut self, mut data: T, key: usize) -> Result<T, HeapError> {
        if self.size == 0 {
            return Err(HeapError::HeapEmpty);
        }
        if self.heap[0].key < key {
            data = self.heap[0].data.clone();
            self.down_heap(0);
        }

        Ok(data)
    }

    fn search(&self, key: usize) -> Result<T, HeapError> {
        let mut current_index = 0usize;
        let mut left_child_larger = false;

        if self.size == 0 {
            return Err(HeapError::HeapEmpty);
        }
        loop {
            if self.heap[current_index].key == key {
                return Ok(self.heap[current_index].data.clone());
            } else if self.heap[current_index].key > key {
                if left_child_larger == true {
                    return Err(HeapError::KeyNotFound);
                } else {
                    left_child_larger = true;
                    current_index += 1;
                }
            } else {
                current_index += 1;
                if current_index == self.size {
                    return Err(HeapError::KeyNotFound);
                }
            }
        }
    }

    fn size(&self) -> usize {
        self.size
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }
}

pub struct BinaryMaxHeap<T> {
    heap: Vec<BinaryHeapNode<T>>,
    size: usize,
}

impl<T> BinaryMaxHeap<T> {
    pub fn new(size: Option<usize>) -> Self {
        match size {
            None => BinaryMaxHeap {
                heap: Vec::<BinaryHeapNode<T>>::with_capacity(100),
                size: 0,
            },
            Some(_) => BinaryMaxHeap {
                heap: Vec::<BinaryHeapNode<T>>::with_capacity(size.unwrap()),
                size: 0,
            },
        }
    }

    fn up_heap(&mut self, node_index: usize) {
        let (mut parent_index, mut current_index) = (0usize, 0usize);
        if node_index != 0 {
            (parent_index, current_index) = ((node_index - 1) / 2, node_index);
        }

        loop {
            if self.heap[current_index].key > self.heap[parent_index].key {
                self.heap.swap(current_index, parent_index);
                if parent_index == 0 {
                    break;
                }
                current_index = parent_index;
                parent_index = (current_index - 1) / 2;
            } else {
                break;
            }
        }
    }

    fn down_heap(&mut self, node_index: usize) {
        let mut current_index: usize = node_index;
        let (mut left_index, mut right_index, mut largest_index) = (
            (current_index * 2) + 1,
            (current_index * 2) + 2,
            current_index,
        );

        loop {
            if (left_index < self.size)
                && (self.heap[left_index].key > self.heap[largest_index].key)
            {
                largest_index = left_index;
            }
            if (right_index < self.size)
                && (self.heap[right_index].key > self.heap[largest_index].key)
            {
                largest_index = right_index;
            }

            if largest_index != current_index {
                self.heap.swap(current_index, largest_index);
                current_index = largest_index;
                (left_index, right_index) = ((current_index * 2) + 1, (current_index * 2) + 2);
            } else {
                break;
            }
        }
    }
}

impl<T: Clone> BinaryHeap<T> for BinaryMaxHeap<T> {
    fn insert(&mut self, data: T, key: usize) {
        self.heap.push(BinaryHeapNode { data, key });
        self.size += 1;
        self.up_heap(self.size - 1);
    }

    fn extract(&mut self) -> Result<T, HeapError> {
        if self.size == 0 {
            return Err(HeapError::KeyNotFound);
        }
        let data = self.heap[0].data.clone();

        self.size -= 1;
        if self.size > 0 {
            self.heap.swap(0, self.size);
            self.down_heap(0);
        }

        Ok(data)
    }

    fn insert_extract(&mut self, mut data: T, key: usize) -> Result<T, HeapError> {
        if self.size == 0 {
            return Err(HeapError::HeapEmpty);
        }
        if self.heap[0].key > key {
            data = self.heap[0].data.clone();
            self.down_heap(0);
        }

        Ok(data)
    }

    fn search(&self, key: usize) -> Result<T, HeapError> {
        let mut current_index = 0usize;
        let mut left_child_smaller = false;

        if self.size == 0 {
            return Err(HeapError::HeapEmpty);
        }
        loop {
            if self.heap[current_index].key == key {
                return Ok(self.heap[current_index].data.clone());
            } else if self.heap[current_index].key < key {
                if left_child_smaller == true {
                    return Err(HeapError::KeyNotFound);
                } else {
                    left_child_smaller = true;
                    current_index += 1;
                }
            } else {
                current_index += 1;
                if current_index == self.size {
                    return Err(HeapError::KeyNotFound);
                }
            }
        }
    }

    fn size(&self) -> usize {
        self.size
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn min_heap_insert() {
        let mut test_heap = BinaryMinHeap::new(Some(2));
        let (test_data_1, test_data_2, test_data_3) = (1, 512, 10240000);
        let (test_key_1, test_key_2, test_key_3) = (10240000, 512, 1);

        assert!(test_heap.size() == 0);
        test_heap.insert(test_data_1.clone(), test_key_1);
        assert!(test_heap.size() == 1);
        test_heap.insert(test_data_2.clone(), test_key_2);
        assert!(test_heap.size() == 2);
        test_heap.insert(test_data_3.clone(), test_key_3);
        assert!(test_heap.size() == 3);
    }

    #[test]
    fn max_heap_insert() {
        let mut test_heap = BinaryMaxHeap::new(Some(2));
        let (test_data_1, test_data_2, test_data_3) = (1, 512, 10240000);
        let (test_key_1, test_key_2, test_key_3) = (10240000, 512, 1);

        assert!(test_heap.size() == 0);
        test_heap.insert(test_data_1.clone(), test_key_1);
        assert!(test_heap.size() == 1);
        test_heap.insert(test_data_2.clone(), test_key_2);
        assert!(test_heap.size() == 2);
        test_heap.insert(test_data_3.clone(), test_key_3);
        assert!(test_heap.size() == 3);
    }

    #[test]
    fn min_heap_extract() {
        let mut test_heap = BinaryMinHeap::new(Some(3));
        let (test_data_1, test_data_2, test_data_3, test_data_4, test_data_5) = (1, 2, 3, 4, 2);
        let (test_key_1, test_key_2, test_key_3, test_key_4, test_key_5) = (1, 2, 3, 400, 2);

        test_heap.insert(test_data_2.clone(), test_key_2);
        test_heap.insert(test_data_4.clone(), test_key_4);
        test_heap.insert(test_data_5.clone(), test_key_5);
        test_heap.insert(test_data_1.clone(), test_key_1);
        test_heap.insert(test_data_3.clone(), test_key_3);

        assert!(test_heap.size() == 5);
        assert!(test_heap.extract().unwrap() == test_data_1);
        assert!(test_heap.size() == 4);
        assert!(test_heap.extract().unwrap() == test_data_2);
        assert!(test_heap.size() == 3);
        assert!(test_heap.extract().unwrap() == test_data_5);
        assert!(test_heap.size() == 2);
        assert!(test_heap.extract().unwrap() == test_data_3);
        assert!(test_heap.size() == 1);
        assert!(test_heap.extract().unwrap() == test_data_4);
        assert!(test_heap.size() == 0);
        assert!(test_heap.extract().is_err());
    }

    #[test]
    fn max_heap_extract() {
        let mut test_heap = BinaryMaxHeap::new(Some(3));
        let (test_data_1, test_data_2, test_data_3, test_data_4, test_data_5) = (1, 2, 3, 4, 3);
        let (test_key_1, test_key_2, test_key_3, test_key_4, test_key_5) = (1, 2, 3, 400, 3);

        test_heap.insert(test_data_3.clone(), test_key_3);
        test_heap.insert(test_data_1.clone(), test_key_1);
        test_heap.insert(test_data_5.clone(), test_key_5);
        test_heap.insert(test_data_4.clone(), test_key_4);
        test_heap.insert(test_data_2.clone(), test_key_2);

        assert!(test_heap.size() == 5);
        assert!(test_heap.extract().unwrap() == test_data_4);
        assert!(test_heap.size() == 4);
        assert!(test_heap.extract().unwrap() == test_data_3);
        assert!(test_heap.size() == 3);
        assert!(test_heap.extract().unwrap() == test_data_5);
        assert!(test_heap.size() == 2);
        assert!(test_heap.extract().unwrap() == test_data_2);
        assert!(test_heap.size() == 1);
        assert!(test_heap.extract().unwrap() == test_data_1);
        assert!(test_heap.size() == 0);
        assert!(test_heap.extract().is_err());
    }

    #[test]
    fn min_heap_insert_extract() {
        let mut test_heap = BinaryMinHeap::new(Some(3));
        let (test_data_1, test_data_2, test_data_3, test_data_4, test_data_5) = (1, 2, 3, 4, 2);
        let (test_key_1, test_key_2, test_key_3, test_key_4, test_key_5) = (1, 2, 3, 400, 2);

        test_heap.insert(test_data_2.clone(), test_key_2);
        test_heap.insert(test_data_4.clone(), test_key_4);
        test_heap.insert(test_data_5.clone(), test_key_5);
        test_heap.insert(test_data_1.clone(), test_key_1);

        assert!(test_heap.size() == 4);
        assert!(test_heap.insert_extract(test_data_3, test_key_3).unwrap() == test_data_1);
        assert!(test_heap.size() == 4);
    }

    #[test]
    fn max_heap_insert_extract() {
        let mut test_heap = BinaryMaxHeap::new(Some(3));
        let (test_data_1, test_data_2, test_data_3, test_data_4, test_data_5) = (1, 2, 3, 4, 3);
        let (test_key_1, test_key_2, test_key_3, test_key_4, test_key_5) = (1, 2, 3, 400, 3);

        test_heap.insert(test_data_3.clone(), test_key_3);
        test_heap.insert(test_data_1.clone(), test_key_1);
        test_heap.insert(test_data_5.clone(), test_key_5);
        test_heap.insert(test_data_4.clone(), test_key_4);

        assert!(test_heap.size() == 4);
        assert!(test_heap.insert_extract(test_data_2, test_key_2).unwrap() == test_data_4);
        assert!(test_heap.size() == 4);
    }

    #[test]
    fn heap_search() {
        let mut test_min_heap = BinaryMinHeap::new(None);
        let mut test_max_heap = BinaryMaxHeap::new(None);
        let (test_data_1, test_data_2, test_data_3) = (1, 2, 3);
        let (test_key_1, test_key_2, test_key_3) = (1, 2, 3);

        assert!(test_min_heap.search(test_key_1).is_err());
        assert!(test_max_heap.search(test_key_1).is_err());

        test_min_heap.insert(test_data_3.clone(), test_key_3);
        test_min_heap.insert(test_data_2.clone(), test_key_2);
        test_min_heap.insert(test_data_1.clone(), test_key_1);
        test_max_heap.insert(test_data_1.clone(), test_key_1);
        test_max_heap.insert(test_data_2.clone(), test_key_2);
        test_max_heap.insert(test_data_3.clone(), test_key_3);

        assert!(test_min_heap.size() == 3);
        assert!(test_min_heap.search(test_key_1).unwrap() == test_data_1);
        assert!(test_min_heap.size() == 3);
        assert!(test_min_heap.search(test_key_2).unwrap() == test_data_2);
        assert!(test_min_heap.size() == 3);
        assert!(test_min_heap.search(test_key_3).unwrap() == test_data_3);
        assert!(test_min_heap.size() == 3);

        assert!(test_max_heap.size() == 3);
        assert!(test_max_heap.search(test_key_1).unwrap() == test_data_1);
        assert!(test_max_heap.size() == 3);
        assert!(test_max_heap.search(test_key_2).unwrap() == test_data_2);
        assert!(test_max_heap.size() == 3);
        assert!(test_max_heap.search(test_key_3).unwrap() == test_data_3);
        assert!(test_max_heap.size() == 3);
    }

    #[test]
    fn heap_is_empty() {
        let mut test_min_heap = BinaryMinHeap::new(None);
        let mut test_max_heap = BinaryMaxHeap::new(None);
        let (test_data_1, test_data_2, test_data_3) = (1, 512, 10240000);
        let (test_key_1, test_key_2, test_key_3) = (10240000, 512, 1);

        assert!(test_min_heap.is_empty() == true);
        test_min_heap.insert(test_data_1.clone(), test_key_3);
        test_min_heap.insert(test_data_2.clone(), test_key_2);
        test_min_heap.insert(test_data_3.clone(), test_key_1);
        assert!(test_min_heap.is_empty() == false);
        let _ = test_min_heap.extract();
        let _ = test_min_heap.extract();
        let _ = test_min_heap.extract();
        assert!(test_max_heap.is_empty() == true);

        assert!(test_max_heap.is_empty() == true);
        let _ = test_max_heap.insert(test_data_1.clone(), test_key_3);
        let _ = test_max_heap.insert(test_data_2.clone(), test_key_2);
        let _ = test_max_heap.insert(test_data_3.clone(), test_key_1);
        assert!(test_max_heap.is_empty() == false);
        let _ = test_max_heap.extract();
        let _ = test_max_heap.extract();
        let _ = test_max_heap.extract();
        assert!(test_max_heap.is_empty() == true);
    }
}
