
/// An iterator to iterate through all the combinations of pairs in a **Clone**-able iterator.
pub struct CombinatePair<I: Iterator> {
    iter: I,
    next_iter: Option<I>,
    val: Option<I::Item>,
}
impl<I> CombinatePair<I> where I: Iterator {
    /// Create a new **CombinatePair** from a clonable iterator.
    pub fn new(iter: I) -> CombinatePair<I> {
        CombinatePair { 
            iter: iter, 
            next_iter: None, 
            val: None,
        }
    }
}

impl<I> Iterator for CombinatePair<I> where I: Iterator + Clone, I::Item: Clone{
    type Item = (I::Item, I::Item);
    fn next(&mut self) -> Option<Self::Item> {
        // not having a value means we iterate once more throug the first iterator
        if self.val.is_none() {
            self.val = self.iter.next();
            self.next_iter = Some(self.iter.clone());
        }
        // if its still none, we're out of values
        if self.val.is_none() {
            return None;
        }
        

        let ret_ele = self.val.clone().unwrap();
        match self.next_iter.as_mut().unwrap().next() {
            Some(ref x) => {
                return Some((ret_ele, x.clone()));
            },
            None => {
                self.val = None;
            }
        }
        // try again if we ran out of values in the second iterator
        self.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let (_, hi) = self.iter.size_hint();
        match self.next_iter {
            Some(ref i) => {
                let (low, hi_next) = i.size_hint();
                (low, hi.and_then(|x| hi_next.and_then(|y| Some(x * y))))
            },
            None => (0, hi),
        }
    }
}
pub trait Itertools : Iterator {
    fn combinate_pair(self) -> CombinatePair<Self> where
        Self: Sized + Clone, Self::Item: Clone
    {
        CombinatePair::new(self)
    }

}

impl<T: ?Sized> Itertools for T where T: Iterator { }