

// use std::slice::IterMut;




pub trait CxxIterator {
    type Item;
    fn not_equal(&self, rhs: &Self) -> bool;
    fn next(&mut self) -> &mut Self;
    fn get(&mut self) -> Self::Item;
}

pub struct Iterate<T>
where
    T: CxxIterator,
{
    pub begin: T,
    pub end: T,
    pub started: bool,
}

impl<T> Iterate<T>
where
    T: CxxIterator,
{
    pub fn new(begin: T, end: T) -> Self {
        Self {
            begin,
            end,
            started: false,
        }
    }
}

impl<T> Iterator for Iterate<T>
where
    T: CxxIterator,
{
    type Item = T::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.started {
            self.begin.next();
        }

        if self.begin.not_equal(&self.end) {
            self.started = true;
            Some(self.begin.get())
        } else {
            self.started = false;
            None
        }
    }
}
