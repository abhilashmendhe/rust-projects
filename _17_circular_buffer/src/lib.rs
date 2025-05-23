use std::marker::PhantomData;

#[derive(Debug)]
pub struct CircularBuffer<T> {
    // We fake using T here, so the compiler does not complain that
    // "parameter `T` is never used". Delete when no longer needed.
    buff: Vec<T>,
    cap: usize,
    count: usize,
    read_marker: usize, 
    write_marker: usize,
    phantom: std::marker::PhantomData<T>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T: Clone + Default> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {

        CircularBuffer { 
            buff: vec![T::default(); capacity], 
            cap: capacity,
            count: 0,
            read_marker: 0,
            write_marker: 0,
            phantom: PhantomData 
        }
    }

    pub fn write(&mut self, _element: T) -> Result<(), Error> {
        if self.count == self.cap {
            Err(Error::FullBuffer)
        } else {

            self.buff[self.write_marker] = _element;
            self.write_marker = (self.write_marker + 1) % self.cap;
            self.count += 1;
            Ok(())
        }
    }

    pub fn read(&mut self) -> Result<T, Error> {
        if self.count == 0 {
            Err(Error::EmptyBuffer)
        } else {
            let value = self.buff[self.read_marker].clone();
            self.read_marker = (self.read_marker + 1) % self.cap;
            self.count -= 1;
            Ok(value)
        }
    }

    pub fn clear(&mut self) {
        self.read_marker = 0;
        self.write_marker = 0;
        self.count = 0;
        self.buff = vec![T::default(); self.cap];
    }

    pub fn overwrite(&mut self, _element: T) {
        self.buff[self.write_marker] = _element;
        self.write_marker = (self.write_marker + 1) % self.cap;
        if self.count == self.cap {
            self.read_marker = (self.read_marker + 1) % self.cap;
        }
        if self.count < self.cap {
            self.count += 1;
        }
    }
}

