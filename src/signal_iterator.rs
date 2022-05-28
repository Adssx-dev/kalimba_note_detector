use crate::signal::Signal;

pub struct SignalIterator<'a> {
    signal : &'a Signal,
    window_size : usize,
    index : usize,
}

impl SignalIterator<'_> {
    pub fn new(signal : & Signal, window_size : usize) -> SignalIterator {
        SignalIterator { 
            signal, 
            window_size, 
            index: 0
        }
    }
}

impl Iterator for SignalIterator<'_> {
    type Item = Signal;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.signal.samples.len() - self.window_size {
            self.index += 1;
            Some(Signal::from_vector(&(self.signal.samples[self.index..(self.index + self.window_size)]), self.signal.basis_definition.clone()))
        } else {
            None
        }
    }
}





