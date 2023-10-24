use std::{cell::Cell, cmp::Ordering, rc::Rc};

use rand::Rng;


#[derive(Clone)]
pub struct SortEvaluator<T> {
    pub t: T,
    pub cmps: Rc<Cell<usize>>,
}

impl<T: Ord + Clone> SortEvaluator<T> {
    pub fn generate_values(
        n: usize,
        counter: &Rc<Cell<usize>>,
        rand: &mut rand::prelude::ThreadRng,
    ) -> Vec<SortEvaluator<usize>> {
        let mut values = Vec::with_capacity(n);
        for _ in 0..n {
            values.push(SortEvaluator {
                t: rand.gen::<usize>(),
                cmps: Rc::clone(&counter),
            });
        }
        values
    }
}

impl<T: PartialEq> PartialEq for SortEvaluator<T> {
    fn eq(&self, other: &Self) -> bool {
        self.cmps.set(self.cmps.get() + 1);
        self.t == other.t
    }
}
impl<T: Eq> Eq for SortEvaluator<T> {}

impl<T: PartialOrd> PartialOrd for SortEvaluator<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.cmps.set(self.cmps.get() + 1);
        self.t.partial_cmp(&other.t)
    }
}
impl<T: Ord> Ord for SortEvaluator<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cmps.set(self.cmps.get() + 1);
        self.t.cmp(&other.t)
    }
}
