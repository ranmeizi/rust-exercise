#[derive(Debug)]
pub struct MinHeap {
    array: Vec<usize>,
}

impl MinHeap {
    pub fn new() -> MinHeap {
        MinHeap { array: vec![] }
    }

    pub fn from(array: Vec<usize>) -> MinHeap {
        let mut heap = MinHeap::new();

        heap.init(array);

        heap
    }

    pub fn size(&self) -> usize {
        self.array.len()
    }

    pub fn init(&mut self, array: Vec<usize>) {
        for el in array.iter() {
            self.push(*el)
        }
    }

    pub fn push(&mut self, num: usize) {
        self.array.push(num);
        self.percolate_up(self.array.len() - 1)
    }

    pub fn pop(&mut self) -> usize {
        let item = self.array[0];

        self.swap(0, self.array.len() - 1);

        self.array.pop();

        self.percolate_down(0);

        item
    }

    fn left_son(&self, index: usize) -> Option<usize> {
        let res = index * 2 + 1;
        if res < self.array.len() {
            Some(res)
        } else {
            None
        }
    }

    fn right_son(&self, index: usize) -> Option<usize> {
        let res = index * 2 + 2;
        if res < self.array.len() {
            Some(res)
        } else {
            None
        }
    }

    fn parent(&self, index: usize) -> Option<usize> {
        if index == 0 {
            None
        } else {
            Some(index / 2)
        }
    }

    fn swap(&mut self, i1: usize, i2: usize) {
        let temp = *self.array.get(i1).unwrap();
        self.array[i1] = self.array[i2];
        self.array[i2] = temp;
    }

    fn percolate_up(&mut self, index: usize) {
        if self.parent(index) == None {
            return;
        }
        let p_index = self.parent(index).unwrap();
        let p = self.array.get(p_index).unwrap();
        let curr = self.array.get(index).unwrap();

        if p > curr {
            self.swap(p_index, index);
            self.percolate_up(p_index);
        }
    }

    fn percolate_down(&mut self, index: usize) {
        let left_index = self.left_son(index);
        let right_index = self.right_son(index);

        if (left_index, right_index).eq(&(None, None)) {
            return;
        }

        let (son_index, son) = if left_index == None {
            (
                right_index.unwrap(),
                self.array.get(right_index.unwrap()).unwrap(),
            )
        } else if right_index == None {
            (
                left_index.unwrap(),
                self.array.get(left_index.unwrap()).unwrap(),
            )
        } else {
            let left = self.array.get(left_index.unwrap()).unwrap();
            let right = self.array.get(right_index.unwrap()).unwrap();
            if left > right {
                (right_index.unwrap(), right)
            } else {
                (left_index.unwrap(), left)
            }
        };

        let curr = self.array.get(index).unwrap();

        if curr > son {
            self.swap(index, son_index);
            self.percolate_down(son_index);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut heap = MinHeap::from(vec![5, 4, 3, 2, 1]);
        let mut sortArr = Vec::<usize>::new();

        while heap.size() > 0 {
            sortArr.push(heap.pop())
        }

        assert_eq!(sortArr, vec![1, 2, 3, 4, 5])
    }

    #[test]
    fn test2() {
        let mut heap = MinHeap::from(vec![90, 87, 61, 69, 31, 9, 23, 11]);
        let mut sortArr = Vec::<usize>::new();

        while heap.size() > 0 {
            sortArr.push(heap.pop())
        }

        println!("let me kan kan {:?}", heap);

        assert_eq!(sortArr, vec![9, 11, 23, 31, 61, 69, 87, 90])
    }
}