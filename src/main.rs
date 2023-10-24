use crossterm::event::{Event, KeyCode};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use crossterm::{event, execute};
use rust_sort::sort_evaluator::SortEvaluator;
use rust_sort::Sorter;
use std::cell::Cell;
use std::error::Error;
use std::io::{self, Stdout};
use std::rc::Rc;
use std::time::Duration;

fn main() {}

fn bench<T: Ord + Clone, S: Sorter>(
    sorter: S,
    values: &mut [SortEvaluator<T>],
    counter: &Cell<usize>,
) -> (usize, f64) {
    counter.set(0);
    let time = std::time::Instant::now();
    sorter.sort(values);
    let took = time.elapsed();
    let count = counter.get();
    for i in 1..values.len() {
        assert!(values[i] >= values[i - 1]);
    }
    (count, took.as_secs_f64())
}

fn test_algorithm<T: Ord + Clone, S: Sorter>(
    sorter: S,
    values: &[SortEvaluator<T>],
    counter: &Cell<usize>,
) -> Result<(usize, f64), Box<dyn Error>> {
    let result = bench(sorter, &mut values.to_vec(), counter);
    Ok(result)
}
