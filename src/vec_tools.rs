
pub trait GetMutPair<A> {
    fn get_pair_mut(&mut self, i: usize, j: usize) -> (& mut A, & mut A);
}

impl<A> GetMutPair<A> for Vec<A> {

    fn get_pair_mut(& mut self, i: usize, j: usize) -> (& mut A, & mut A) {
        if i == j
        {
            panic!("Trying to borrow the same index twice.");
        } else if i > j {
            let (ref_j, ref_i) = self.split_at_mut(i);
            (& mut ref_i[0], & mut ref_j[j])
        } else {
            let (ref_i, ref_j) = self.split_at_mut(j);
            (& mut ref_i[i], & mut ref_j[0])
        }
    }
}
