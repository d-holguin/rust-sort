use std::{cell::Cell, cmp::Ordering, error::Error, sync::Arc};

use rand::Rng;

use crate::sorter::Sorter;

#[derive(Clone)]
pub struct SortEvaluator<T> {
    pub t: T,
    pub cmps: Arc<Cell<usize>>,
}

impl<T: Ord + Clone> SortEvaluator<T> {
    pub fn generate_values(
        n: usize,
        counter: &Arc<Cell<usize>>,
        rand: &mut rand::prelude::ThreadRng,
    ) -> Vec<SortEvaluator<usize>> {
        let mut values = Vec::with_capacity(n);
        for _ in 0..n {
            values.push(SortEvaluator {
                t: rand.gen::<usize>(),
                cmps: Arc::clone(&counter),
            });
        }
        values
    }

    pub fn bench<S: Sorter>(
        sorter: &S,
        values: &mut [SortEvaluator<T>],
        counter: &Cell<usize>,
    ) -> Result<SortMetrics, Box<dyn Error>> {
        counter.set(0);
        let time = std::time::Instant::now();
        sorter.sort(values);
        let took = time.elapsed();
        let count = counter.get();
        for i in 1..values.len() {
            assert!(values[i] >= values[i - 1]);
        }
        Ok(SortMetrics {
            sorter: format!("{:?}", sorter),
            count,
            time: took.as_secs_f64(),
            total_values: values.len(),
        })
    }
}
#[derive(Debug)]
pub struct SortMetrics {
    pub sorter: String,
    pub count: usize,
    pub time: f64,
    pub total_values: usize,
}

impl<T: Eq> Eq for SortEvaluator<T> {}

impl<T: PartialEq> PartialEq for SortEvaluator<T> {
    fn eq(&self, other: &Self) -> bool {
        self.cmps.set(self.cmps.get() + 1);
        self.t == other.t
    }
}

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
