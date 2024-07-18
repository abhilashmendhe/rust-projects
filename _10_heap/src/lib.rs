
#[derive(Debug)]
pub enum HeapType {
    MAX,
    MIN
}

#[derive(Debug)]
pub struct Heap {
    pub heap_type: HeapType,
    capacity: usize,
    heap_ptr: usize,
    arr: Vec<i32>
}
impl Heap {
    pub fn min_heap(capacity: usize) -> Self {
        Heap {
            heap_type: HeapType::MIN,
            capacity,
            heap_ptr: 0,
            arr: vec![i32::MAX; capacity]
        }
    }
    pub fn max_heap(capacity: usize) -> Self {
        Heap {
            heap_type: HeapType::MAX,
            capacity,
            heap_ptr: 0,
            arr: vec![i32::MIN; capacity]
        }
    }
    fn sift_up_min_heap(&mut self) {

        let mut ptr = self.heap_ptr;
        let mut parent = (ptr - 1) / 2;
        while self.arr[ptr] < self.arr[parent] {
            (self.arr[ptr], self.arr[parent]) = (self.arr[parent], self.arr[ptr]); 
            ptr = parent;
            if  ptr.checked_sub(1) == None {
                
                break;
            }
            parent = (ptr - 1) / 2;
        }
    }
    fn sift_up_max_heap(&mut self) {

        let mut ptr = self.heap_ptr;
        let mut parent = (ptr - 1) / 2;
        while self.arr[ptr] > self.arr[parent] {
            (self.arr[ptr], self.arr[parent]) = (self.arr[parent], self.arr[ptr]); 
            ptr = parent;
            if  ptr.checked_sub(1) == None {
                break;
            }
            parent = (ptr - 1) / 2;
        }
    }
    fn sift_down_min_heap(&mut self, mut ind: usize) {
        
        let mut l_child = 2 * ind + 1;
        let mut r_child = 2 * ind + 2;
        while l_child < self.heap_ptr && r_child < self.heap_ptr {

            if self.arr[l_child] < self.arr[r_child] {
                if self.arr[ind] < self.arr[l_child] {
                    break;
                }
                (self.arr[ind], self.arr[l_child]) = (self.arr[l_child],self.arr[ind]);
                ind = l_child;
            } else {
                if self.arr[ind] < self.arr[r_child] {
                    break;
                }
                (self.arr[ind], self.arr[r_child]) = (self.arr[r_child],self.arr[ind]);
                ind = r_child;
            }
            l_child = 2 * ind + 1;
            r_child = 2 * ind + 2;
        }
        if self.heap_ptr == 2 {
            if self.arr[ind] > self.arr[l_child] {
                (self.arr[ind], self.arr[l_child]) = (self.arr[l_child],self.arr[ind]);
            }
        }
    }

    fn sift_down_max_heap(&mut self, mut ind: usize) {
        let mut l_child = 2 * ind + 1;
        let mut r_child = 2 * ind + 2;
        while l_child < self.heap_ptr && r_child < self.heap_ptr {

            if self.arr[l_child] > self.arr[r_child] {
                if self.arr[ind] > self.arr[l_child] {
                    break;
                }
                (self.arr[ind], self.arr[l_child]) = (self.arr[l_child],self.arr[ind]);
                ind = l_child;
            } else {
                if self.arr[ind] > self.arr[r_child] {
                    break;
                }
                (self.arr[ind], self.arr[r_child]) = (self.arr[r_child],self.arr[ind]);
                ind = r_child;
            }
            l_child = 2 * ind + 1;
            r_child = 2 * ind + 2;
        }
        if self.heap_ptr == 2 {
            if self.arr[ind] < self.arr[l_child] {
                (self.arr[ind], self.arr[l_child]) = (self.arr[l_child],self.arr[ind]);
            }
        }
    }
    
    pub fn print(&self) {
        for i in 0..self.heap_ptr {
            print!("{}, ",self.arr[i]);
        }
        println!();
    }
}

impl  Heap {
    pub fn insert(&mut self, elem: i32) {
        
        let ptr = self.heap_ptr;
        if self.heap_ptr >= self.capacity {
            println!("Heap size execeeded!! Can't add any {} element",elem);
            return;
        }
        self.arr[ptr] = elem;
    
        if  ptr.checked_sub(1) == None {
            self.heap_ptr += 1;
            return;
        }
        match self.heap_type {

            HeapType::MIN => {
                self.sift_up_min_heap();
            },

            HeapType::MAX => {
                self.sift_up_max_heap();
            },
        }
        self.heap_ptr += 1;
            
    }

    pub fn peek(&self) -> Option<i32> {

        if self.heap_ptr == 0 {
            None
        } else {
            Some(self.arr[0])
        }
    }

    pub fn extract(&mut self) -> Option<i32> {
        // println!("{:?}",self);
        if self.heap_ptr == 0 {
            println!("Heap is empty!");
            return None;
        }
        // let last_elem = self.arr[self.heap_ptr - 1];
        let first_elem = self.arr[0];
        (self.arr[0], self.arr[self.heap_ptr - 1]) = (self.arr[self.heap_ptr - 1], self.arr[0]);
        self.heap_ptr -= 1;


        match self.heap_type {

            HeapType::MIN => {
                self.sift_down_min_heap(0);
            },

            HeapType::MAX => {
                self.sift_down_max_heap(0);
            },
        }
        Some(first_elem)
    }
}

